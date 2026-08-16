//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1209/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1209(t1109: f64, t13290: f64, t1161: f64, t12130: f64, t13105: f64, t13171: f64, t13173: f64, t13205: f64, t13217: f64, t13220: f64, t13662: f64, t13688: f64, t2376: f64, t2408: f64, t2409: f64, t28394: f64, t3052: f64, t36041: f64, t3920: f64, t3921: f64, t39466: f64, t43466: f64, t43487: f64, t43872: f64, t829: f64, t830: f64, t831: f64, t833: f64, t8589: f64, t9241: f64, t9820: f64, t9849: f64, t9890: f64) -> (f64, f64) {
    let t49022 = t13290 * t1109;
    let t49058 = 11.0_f64 / 96.0_f64 * t36041 * t13662 - t28394 * t829 * t830 * t831 * t49022 / 16.0_f64 + t12130 * t829 * t830 * t831 * t13171 / 24.0_f64 + t13173 * t3920 * t833 / 32.0_f64 + 3.0_f64 / 8.0_f64 * t3921 * t9820 - t13688 * t3052 / 6.0_f64 - t9849 * t13217 / 32.0_f64 - 7.0_f64 / 72.0_f64 * t43466 - 7.0_f64 / 12.0_f64 * t43487 + t2408 * t2409 * t8589 * t13205 / 4.0_f64 + t9241 * t2409 * t2376 * t13220 * t1161 + t43872 * t13105 / 16.0_f64 + t39466 * t13662 / 16.0_f64 - t3921 * t9890 / 8.0_f64;
    (t49022, t49058)
}
