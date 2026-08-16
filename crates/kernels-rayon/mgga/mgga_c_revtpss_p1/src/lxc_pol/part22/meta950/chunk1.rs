//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3192/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3192(t13089: f64, t5381: f64, t1284: f64, t17306: f64, t3624: f64, t12916: f64, t17704: f64, t5340: f64, t12898: f64, t1804: f64, t12948: f64, t17529: f64) -> (f64, f64, f64, f64, f64) {
    let t59408 = t5381 * t13089;
    let t59411 = t17306 * t1284 * t3624;
    let t59415 = t5340 * t12916 * t17704;
    let t59419 = t1804 * t12898;
    let t59423 = t17529 * t12948;
    (t59408, t59411, t59415, t59419, t59423)
}
