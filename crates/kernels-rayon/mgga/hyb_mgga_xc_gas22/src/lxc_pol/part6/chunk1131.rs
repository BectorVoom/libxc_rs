//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1131/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1131(t7: f64, t11186: f64, t436: f64, t1514: f64, t3613: f64, t1023: f64, t4458: f64, t3814: f64, t7281: f64, t2680: f64, t3804: f64, t1794: f64, t224: f64, t3619: f64, t545: f64, t9909: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t11187 = t11186 * t436;
    let t11188 = t3613 * t1514;
    let t11190 = t1023 * t4458;
    let t11192 = t7281 * t3814;
    let t11197 = t2680 * t3804;
    let t11203 = piecewise3(t8, 0.0_f64, -8.0_f64 / 27.0_f64 * t11192 * t545 + 16.0_f64 / 9.0_f64 * t3619 * t1794 + 4.0_f64 / 9.0_f64 * t11197 * t545 + 4.0_f64 / 3.0_f64 * t224 * t9909);
    (t11187, t11188, t11190, t11192, t11197, t11203)
}
