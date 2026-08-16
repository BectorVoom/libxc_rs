//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1044/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1044(t1145: f64, t9778: f64, t1128: f64, t3785: f64, t1535: f64, t2893: f64, t3788: f64, t513: f64, t2889: f64, t1540: f64, t1543: f64, t1552: f64, t2868: f64, t2875: f64, t2881: f64, t2903: f64, t2922: f64, t2927: f64, t2940: f64, t3706: f64, t3714: f64, t3778: f64, t510: f64, t7764: f64, t9475: f64, t9765: f64, t9766: f64, t9769: f64, t9770: f64, t9773: f64) -> (f64, f64, f64) {
    let t9779 = t1145 * t9778;
    let t9782 = t3785 * t1128;
    let t9785 = t1535 * t2893;
    let t9786 = t1145 * t9785;
    let t9793 = t3788 * t513;
    let t9796 = t1535 * t2889;
    let t9810 = 24.0_f64 * t9765 * t9766 - 360.0_f64 * t9769 * t9770 + 504.0_f64 * t9773 * t9766 - 2.0_f64 * t7764 * t1552 + 21.0_f64 * t2875 * t9779 - 4.0_f64 * t9782 * t3706 - 2.0_f64 * t2927 * t9786 - 2.0_f64 * t2927 * t9475 + 3.0_f64 * t2881 * t9779 - 200.0_f64 / 9.0_f64 * t9793 * t3714 + 15.0_f64 * t2868 * t1145 * t9796 - 18.0_f64 * t2922 * t9786 + 6.0_f64 * t510 * t3778 * t2893 - 4.0_f64 * t2940 * t1540 + 30.0_f64 * t2903 * t1543 * t2889;
    (t9782, t9793, t9810)
}
