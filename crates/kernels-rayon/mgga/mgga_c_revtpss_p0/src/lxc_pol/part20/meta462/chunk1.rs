//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1757/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1757(t239: f64, t47201: f64, t820: f64, t1353: f64, t1388: f64, t1390: f64, t3934: f64, t3936: f64, t3944: f64, t46479: f64, t46483: f64, t46682: f64, t46918: f64, t46922: f64, t46924: f64, t46931: f64, t46934: f64, t46941: f64, t46944: f64, t46947: f64, t46949: f64, t46951: f64, t47188: f64, t47195: f64, t47199: f64, t5671: f64, t5673: f64, t5675: f64, t800: f64, t828: f64, t9699: f64, t9805: f64, t9810: f64, t9826: f64, t9955: f64, t9993: f64) -> f64 {
    let t47203 = t820 * t47201 * t239;
    let t47212 = 0.17149607247227894789e-2_f64 * t5671 * t5673 * t46682 * t5675 + 0.27210710165601593064e0_f64 * t46918 + 0.6098400337114239387e-3_f64 * t46922 + 0.51448821741683684368e-1_f64 * t5671 * t9955 * t9826 * t46924 - 0.30492001685571196934e-4_f64 * t46931 + 0.15246000842785598467e-4_f64 * t46934 + t3944 * t800 * t9699 * t1353 / 4.0_f64 + 0.15246000842785598467e-4_f64 * t46941 + 0.5421477899694558815e-3_f64 * t46944 - 0.18295201011342718161e-3_f64 * t46947 - 7.0_f64 / 4.0_f64 * t46949 - 0.10289764348336736873e-1_f64 * t5671 * t3936 * t9826 * t46951 + 0.51448821741683684366e-2_f64 * t3934 * t3936 * t9805 * t9810 - 0.21437009059034868486e-3_f64 * t1388 * t1390 * t828 * t47188 + 0.24009450146119052705e-1_f64 * t47195 - 0.51384669507166276316e-2_f64 * t47199 + 0.51448821741683684368e-2_f64 * t47203 * t1390 * t828 * t46479 - 0.77173232612525526552e-2_f64 * t9993 * t1390 * t828 * t46483;
    t47212
}
