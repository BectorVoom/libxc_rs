//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2283/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2283(t18029: f64, t6754: f64, t1025: f64, t1618: f64, t1622: f64, t17693: f64, t17697: f64, t17734: f64, t23537: f64, t23544: f64, t25577: f64, t25580: f64, t4636: f64, t4652: f64, t5900: f64, t6765: f64, t82914: f64, t88277: f64, t88305: f64, t88307: f64, t88388: f64) -> f64 {
    let t99539 = t18029 * t6754;
    let t99556 = t88305 - t88307 - t82914 / 6912.0_f64 - t23544 * t5900 / 1152.0_f64 + t99539 * t1025 / 1536.0_f64 + t88388 * t1618 / 768.0_f64 + t25577 * t4652 / 768.0_f64 + t88277 * t1622 / 1152.0_f64 + t25580 * t4636 / 1152.0_f64 + 5.0_f64 / 3456.0_f64 * t6765 * t17693 + 5.0_f64 / 2592.0_f64 * t6765 * t17697 + t23537 * t17734 / 384.0_f64;
    t99556
}
