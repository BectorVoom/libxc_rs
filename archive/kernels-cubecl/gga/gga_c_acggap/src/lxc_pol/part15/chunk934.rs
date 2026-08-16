//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 934/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk934<F: Float>(t157: F, t406: F, t847: F, t2248: F, t469: F, t103: F, t2236: F, t30005: F, t3054: F, t633: F, t865: F, t2245: F, t7924: F) -> (F, F, F, F, F, F) {
    let t32194 = t847 * t406 * t157;
    let t32262 = t2248 * t469;
    let t32278 = t103 * t2248;
    let t32315 = t30005 * t2236;
    let t32324 = F::cast_from(0.39512695097613069591e1_f64) * t3054 * t633 * t865;
    let t32329 = t7924 * t2245;
    (t32194, t32262, t32278, t32315, t32324, t32329)
}
