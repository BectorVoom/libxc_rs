//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1011/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1011(t1117: f64, t1134: f64, t1145: f64, t1546: f64, t2869: f64, t2876: f64, t2889: f64, t2893: f64, t2903: f64, t3713: f64, t3717: f64, t3760: f64, t3767: f64, t3772: f64, t3788: f64, t518: f64, t7721: f64, t7739: f64, t7769: f64, t7780: f64, t9419: f64, t9436: f64, t9441: f64, t9444: f64, t9449: f64, t9453: f64, t9458: f64) -> f64 {
    let t9463 = -4.0_f64 * t1117 * t9419 - 36.0_f64 * t1134 * t3760 * t2893 - 36.0_f64 * t1134 * t1546 * t2889 + 42.0_f64 * t518 * t3767 * t2893 - 8.0_f64 * t3788 * t3772 - 180.0_f64 * t2903 * t1546 * t2869 - 336.0_f64 * t518 * t9436 * t2876 + 12.0_f64 * t1117 * t9441 - 90.0_f64 * t7721 * t1145 * t9444 - 168.0_f64 * t7780 * t9449 + 6.0_f64 * t7739 * t9453 - 12.0_f64 * t7769 * t9449 + 800.0_f64 / 27.0_f64 * t3713 * t9458 + 800.0_f64 / 27.0_f64 * t3717 * t9458;
    t9463
}
