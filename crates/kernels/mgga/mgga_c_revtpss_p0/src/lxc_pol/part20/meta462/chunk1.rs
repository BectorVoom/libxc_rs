//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1757/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1757<F: Float>(t239: F, t47201: F, t820: F, t1353: F, t1388: F, t1390: F, t3934: F, t3936: F, t3944: F, t46479: F, t46483: F, t46682: F, t46918: F, t46922: F, t46924: F, t46931: F, t46934: F, t46941: F, t46944: F, t46947: F, t46949: F, t46951: F, t47188: F, t47195: F, t47199: F, t5671: F, t5673: F, t5675: F, t800: F, t828: F, t9699: F, t9805: F, t9810: F, t9826: F, t9955: F, t9993: F) -> F {
    let t47203 = t820 * t47201 * t239;
    let t47212 = F::cast_from(0.17149607247227894789e-2_f64) * t5671 * t5673 * t46682 * t5675 + F::cast_from(0.27210710165601593064e0_f64) * t46918 + F::cast_from(0.6098400337114239387e-3_f64) * t46922 + F::cast_from(0.51448821741683684368e-1_f64) * t5671 * t9955 * t9826 * t46924 - F::cast_from(0.30492001685571196934e-4_f64) * t46931 + F::cast_from(0.15246000842785598467e-4_f64) * t46934 + t3944 * t800 * t9699 * t1353 / F::new(4.0) + F::cast_from(0.15246000842785598467e-4_f64) * t46941 + F::cast_from(0.5421477899694558815e-3_f64) * t46944 - F::cast_from(0.18295201011342718161e-3_f64) * t46947 - F::new(7.0) / F::new(4.0) * t46949 - F::cast_from(0.10289764348336736873e-1_f64) * t5671 * t3936 * t9826 * t46951 + F::cast_from(0.51448821741683684366e-2_f64) * t3934 * t3936 * t9805 * t9810 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t1390 * t828 * t47188 + F::cast_from(0.24009450146119052705e-1_f64) * t47195 - F::cast_from(0.51384669507166276316e-2_f64) * t47199 + F::cast_from(0.51448821741683684368e-2_f64) * t47203 * t1390 * t828 * t46479 - F::cast_from(0.77173232612525526552e-2_f64) * t9993 * t1390 * t828 * t46483;
    t47212
}
