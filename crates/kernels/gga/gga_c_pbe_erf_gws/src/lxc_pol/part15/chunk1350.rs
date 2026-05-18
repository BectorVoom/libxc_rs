//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1350/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1350<F: Float>(t1105: F, t2051: F, t1172: F, t13756: F, t14149: F, t14153: F, t14825: F, t14831: F, t320: F, t3944: F, t3946: F, t4062: F, t50839: F, t50868: F, t50870: F, t52056: F, t52061: F, t52799: F, t52884: F, t52887: F, t52935: F, t52978: F, t53026: F, t53078: F, t53133: F, t53184: F, t53232: F, t53288: F, t53348: F, t53390: F, t53429: F, t53483: F, t53522: F, t53569: F, t53613: F, t53657: F, t53697: F, t53744: F, t53787: F, t53834: F, t53872: F, t53912: F, t53949: F, t53987: F, t54449: F, t54493: F, t54539: F, t54583: F, t54620: F, t54660: F, t54704: F, t54738: F, t810: F, t8759: F, t8804: F, t945: F) -> F {
    let t54753 = t1105 * t2051;
    let t54761 = F::new(12.0) * t13756 * t3944 * t8804 + F::new(6.0) * t50868 - F::new(2.0) * t50870 + F::new(4.0) * t4062 * t50839 * t14831 + t52884 + t52887 + t1172 * t320 * (t53348 + t53697 + t53232 + t53613 + t53872 + t54660 + t53390 + t54539 + t53987 + t53912 + t53078 + t54620 + t53522 + t54449 + t53949 + t53429 + t53569 + t53483 + t53133 + t53657 + t53184 + t52935 + t54493 + t54704 + t52978 + t53834 + t53744 + t53026 + t53288 + t54738 + t54583 + t53787) * t945 + F::new(6.0) * t3946 * t52799 * t810 - F::new(6.0) * t3946 * t14149 * t14825 + F::new(6.0) * t3946 * t14153 * t54753 + F::new(3.0) * t52056 + t52061 + F::new(6.0) * t13756 * t3944 * t8759;
    t54761
}
