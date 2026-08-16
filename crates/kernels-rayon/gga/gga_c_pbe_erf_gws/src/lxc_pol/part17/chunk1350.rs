//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1350/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1350(t1105: f64, t2051: f64, t1172: f64, t13756: f64, t14149: f64, t14153: f64, t14825: f64, t14831: f64, t320: f64, t3944: f64, t3946: f64, t4062: f64, t50839: f64, t50868: f64, t50870: f64, t52056: f64, t52061: f64, t52799: f64, t52884: f64, t52887: f64, t52935: f64, t52978: f64, t53026: f64, t53078: f64, t53133: f64, t53184: f64, t53232: f64, t53288: f64, t53348: f64, t53390: f64, t53429: f64, t53483: f64, t53522: f64, t53569: f64, t53613: f64, t53657: f64, t53697: f64, t53744: f64, t53787: f64, t53834: f64, t53872: f64, t53912: f64, t53949: f64, t53987: f64, t54449: f64, t54493: f64, t54539: f64, t54583: f64, t54620: f64, t54660: f64, t54704: f64, t54738: f64, t810: f64, t8759: f64, t8804: f64, t945: f64) -> f64 {
    let t54753 = t1105 * t2051;
    let t54761 = 12.0_f64 * t13756 * t3944 * t8804 + 6.0_f64 * t50868 - 2.0_f64 * t50870 + 4.0_f64 * t4062 * t50839 * t14831 + t52884 + t52887 + t1172 * t320 * (t53348 + t53697 + t53232 + t53613 + t53872 + t54660 + t53390 + t54539 + t53987 + t53912 + t53078 + t54620 + t53522 + t54449 + t53949 + t53429 + t53569 + t53483 + t53133 + t53657 + t53184 + t52935 + t54493 + t54704 + t52978 + t53834 + t53744 + t53026 + t53288 + t54738 + t54583 + t53787) * t945 + 6.0_f64 * t3946 * t52799 * t810 - 6.0_f64 * t3946 * t14149 * t14825 + 6.0_f64 * t3946 * t14153 * t54753 + 3.0_f64 * t52056 + t52061 + 6.0_f64 * t13756 * t3944 * t8759;
    t54761
}
