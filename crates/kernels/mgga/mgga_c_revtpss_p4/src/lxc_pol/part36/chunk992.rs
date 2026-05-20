//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 992/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk992<F: Float>(t231: F, t23244: F, t23168: F, t827: F, t828: F, t23172: F, t124: F, t23114: F, t800: F, t23148: F, t1544: F, t5984: F) -> (F, F, F, F, F, F) {
    let t23245 = t23244 * t231;
    let t23253 = t827 * t828 * t23168;
    let t23257 = t827 * t828 * t23172;
    let t23262 = t124 * t23114;
    let t23263 = t800 * t23262;
    let t23266 = t124 * t23148;
    let t23267 = t800 * t23266;
    let t23275 = t800 * t5984 * t1544;
    (t23245, t23253, t23257, t23263, t23267, t23275)
}
