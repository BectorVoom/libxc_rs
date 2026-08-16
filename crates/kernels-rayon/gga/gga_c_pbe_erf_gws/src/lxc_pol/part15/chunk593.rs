//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 593/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk593(t2685: f64, t639: f64, t1000: f64, t610: f64, t1827: f64, t587: f64, t1684: f64, t1741: f64, t1788: f64, t1028: f64, t395: f64, t1691: f64, t2679: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2687 = 4.0_f64 / 45.0_f64 * t639 * t2685;
    let t2688 = t1000 * t610;
    let t2689 = t1827 * t2688;
    let t2691 = 4.0_f64 / 45.0_f64 * t587 * t2689;
    let t2692 = 4.0_f64 / 45.0_f64 * t1684;
    let t2693 = 4.0_f64 / 45.0_f64 * t1741;
    let t2694 = 4.0_f64 / 45.0_f64 * t1788;
    let t2696 = t395 * t1028;
    let t2698 = t1691 * t2679;
    (t2687, t2688, t2689, t2691, t2692, t2693, t2694, t2696, t2698)
}
