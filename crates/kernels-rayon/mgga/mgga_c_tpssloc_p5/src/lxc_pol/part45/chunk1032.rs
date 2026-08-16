//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1032/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1032(t1874: f64, t91857: f64, t26977: f64, t6525: f64, t22585: f64, t8607: f64, t31304: f64, t7000: f64, t112620: f64, t112621: f64, t112622: f64, t115690: f64, t115695: f64, t115698: f64, t115700: f64, t115702: f64, t115704: f64, t22619: f64, t2323: f64, t23938: f64, t23953: f64, t31246: f64, t31532: f64, t6539: f64, t7042: f64, t7220: f64, t8450: f64) -> f64 {
    let t115708 = 2.0_f64 * t91857 * t1874;
    let t115712 = 4.0_f64 * t26977 * t6525;
    let t115716 = 3.0_f64 * t8607 * t22585;
    let t115718 = 2.0_f64 * t31304 * t7000;
    let t115719 = -4.0_f64 * t22619 * t7042 - 4.0_f64 * t2323 * t31532 - 4.0_f64 * t23938 * t6539 + 3.0_f64 * t23953 * t8450 - 2.0_f64 * t31246 * t7220 - t112620 - t112621 - t112622 + t115690 + t115695 - t115698 + t115700 - t115702 - t115704 - t115708 - t115712 + t115716 - t115718;
    t115719
}
