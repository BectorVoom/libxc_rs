//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 547/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk547(t1006: f64, t583: f64, t1689: f64, t1743: f64, t2696: f64, t2699: f64, t2702: f64, t2707: f64, t203: f64, t184: f64, t221: f64, t1755: f64, t1756: f64, t2760: f64, t2763: f64, t2766: f64, t2770: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2807 = t1006 * t583;
    let t2808 = 4.0_f64 / 45.0_f64 * t2807;
    let t2814 = -t1743 - 0.62972222222222222223e-3_f64 * t1689 - 0.62972222222222222223e-3_f64 * t2696 + 0.12594444444444444445e-2_f64 * t2699 - 0.37783333333333333334e-2_f64 * t2702 - 0.37783333333333333334e-2_f64 * t2707;
    let t2815 = t203 * t2814;
    let t2816 = t2815 * t184;
    let t2818 = 2.0_f64 / 15.0_f64 * t2816 * t221;
    let t2824 = -t1755 - 0.62972222222222222223e-3_f64 * t1756 - 0.62972222222222222223e-3_f64 * t2760 + 0.12594444444444444445e-2_f64 * t2763 - 0.37783333333333333334e-2_f64 * t2766 + 0.37783333333333333334e-2_f64 * t2770;
    (t2807, t2808, t2814, t2815, t2816, t2818, t2824)
}
