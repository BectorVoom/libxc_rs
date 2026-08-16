//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1691/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1691(t11250: f64, t11774: f64, t11927: f64, t15700: f64, t15701: f64, t15707: f64, t16222: f64, t19738: f64, t19741: f64, t23633: f64, t23892: f64, t23900: f64, t23904: f64, t23911: f64, t23964: f64, t3091: f64, t3092: f64, t3117: f64, t43105: f64, t6266: f64, t78676: f64, t78750: f64, t78756: f64, t78763: f64, t78802: f64, t79159: f64, t88773: f64, t88794: f64) -> f64 {
    let t88800 = 0.57165357490759649296e-3_f64 * t78676 - 0.17149607247227894789e-2_f64 * t15707 * t23892 + 0.16937883700965822014e-2_f64 * t78750 + 0.34299214494455789577e-2_f64 * t11774 * t15701 * t23633 * t23911 + 0.28582678745379824648e-2_f64 * t15700 * t16222 * t88773 - 0.34299214494455789578e-2_f64 * t15700 * t15701 * t88773 + 0.51448821741683684368e-2_f64 * t11927 * t3117 * t23964 * t23911 + 0.57165357490759649296e-3_f64 * t3091 * t3092 * t79159 * t6266 + 0.34299214494455789578e-2_f64 * t19738 * t23900 - 0.17149607247227894789e-2_f64 * t19741 * t23904 + 0.19055119163586549765e-2_f64 * t78756 + 0.19055119163586549765e-2_f64 * t78763 + 0.51448821741683684368e-2_f64 * t43105 * t3117 * t88794 * t11250 + 0.34299214494455789578e-2_f64 * t78802;
    t88800
}
