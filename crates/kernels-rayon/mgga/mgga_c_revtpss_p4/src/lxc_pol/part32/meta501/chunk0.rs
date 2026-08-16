//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1787/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1787(t1450: f64, t6816: f64, t6836: f64, t196: f64, t197: f64, t6773: f64, t5920: f64, t94: f64, t21663: f64, t38: f64, t5868: f64, t76: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29494 = t1450 * t6816;
    let t29498 = t1450 * t6836;
    let t29506 = t6773 * t196 * t197;
    let t29508 = t94 * t5920;
    let t29513 = t21663 * t38;
    let t29532 = t76 * t5868;
    (t29494, t29498, t29506, t29508, t29513, t29532)
}
