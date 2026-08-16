//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 942/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk942(t2358: f64, t35770: f64, t3684: f64, t7822: f64, t11125: f64, t2969: f64, t3511: f64, t45163: f64, t45164: f64, t45202: f64, t45257: f64, t45309: f64, t45363: f64, t45412: f64, t45455: f64, t45512: f64, t45570: f64, t45619: f64, t45672: f64, t45718: f64, t45767: f64, t45824: f64, t45875: f64, t45919: f64, t45959: f64, t45967: f64, t45969: f64, t45971: f64, t45973: f64, t45974: f64, t45976: f64, t45978: f64, t45983: f64, t45986: f64, t45988: f64, t45990: f64, t748: f64, t8440: f64) -> (f64, f64) {
    let t45992 = 2.0_f64 * t35770 * t2358;
    let t45993 = t7822 * t3684;
    let t45994 = -t45163 + t45164 - 2.0_f64 * t2969 * t11125 - t748 * (t45202 + t45257 + t45309 + t45363 + t45412 + t45455 + t45512 + t45570 + t45619 + t45672 + t45718 + t45767 + t45824 + t45875 + t45919 + t45959) - t45967 - t45969 - t45971 - t45973 + t45974 + t45976 + t45978 - 2.0_f64 * t8440 * t3511 + t45983 - t45986 + t45988 + t45990 - t45992 - t45993;
    (t45992, t45994)
}
