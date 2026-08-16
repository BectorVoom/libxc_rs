//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1431/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1431(t11552: f64, t2824: f64, t11556: f64, t5204: f64, t9520: f64, t11540: f64, t22746: f64, t30908: f64, t30919: f64, t30993: f64, t30996: f64, t7806: f64, t7811: f64, t9521: f64, t9527: f64, t9535: f64, t9549: f64, t9558: f64) -> f64 {
    let t31004 = t11552 * t2824;
    let t31009 = t11556 * t2824;
    let t31015 = t5204 * t9520;
    let t31030 = -1600.0_f64 / 3.0_f64 * t9549 * t30919 + 32.0_f64 * t7806 * t31004 + 1600.0_f64 / 3.0_f64 * t9549 * t30993 - 112.0_f64 / 3.0_f64 * t9558 * t31009 + 160.0_f64 * t22746 * t11540 * t2824 + 400.0_f64 / 9.0_f64 * t31015 * t9535 - 352.0_f64 / 3.0_f64 * t7806 * t30908 + 32.0_f64 / 9.0_f64 * t7811 * t30996 - 1600.0_f64 / 27.0_f64 * t9521 * t30919 + 32.0_f64 / 9.0_f64 * t7811 * t31004 + 1600.0_f64 / 27.0_f64 * t9521 * t30993 - 16.0_f64 / 3.0_f64 * t9527 * t31009;
    t31030
}
