//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1396/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1396<F: Float>(t5218: F, t1094: F, t1102: F, t2916: F, t3058: F, t3061: F, t15142: F, t18187: F, t15078: F, t15083: F, t17627: F, t18184: F, t18191: F, t4297: F, t5229: F, t53793: F, t53812: F, t53823: F, t53826: F, t53829: F, t53831: F) -> (F, F, F, F) {
    let t58880 = t5218 * t5218;
    let t58884 = F::new(0.35089340384731224426e1) * t1102 * t2916 * t58880 * t1094;
    let t58888 = F::new(0.51947267698127589897e2) * t1102 * t3058 * t58880 * t3061;
    let t58889 = t15142 * t18187;
    let t58905 = t58884 - t58888 - F::new(400.0) / F::new(27.0) * t4297 * t58889 + F::new(80000.0) / F::new(243.0) * t53823 + F::new(200.0) / F::new(81.0) * t53826 - F::new(400.0) / F::new(9.0) * t15083 * t18191 + F::new(8.0) / F::new(9.0) * t53829 + F::new(80000.0) / F::new(81.0) * t53812 * t17627 - F::new(200.0) / F::new(3.0) * t53831 * t5229 - F::new(1520000.0) / F::new(243.0) * t53793 * t17627 + F::new(8.0) / F::new(3.0) * t15078 * t18184;
    (t58880, t58884, t58888, t58905)
}
