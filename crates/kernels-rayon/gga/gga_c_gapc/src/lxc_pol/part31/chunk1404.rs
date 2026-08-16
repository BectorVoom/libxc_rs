//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1404/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1404(t35031: f64, t35036: f64, t35039: f64, t35045: f64, t35048: f64, t35051: f64, t35056: f64, t35059: f64, t35062: f64, t35069: f64, t35071: f64, t35074: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37172 = 0.40483072916666666669e-4_f64 * t35031;
    let t37174 = 0.50680539737635041234e-3_f64 * t35036;
    let t37175 = 0.25301920572916666668e-5_f64 * t35039;
    let t37177 = 0.38673709012042260328e-8_f64 * t35045;
    let t37178 = 0.80966145833333333338e-4_f64 * t35048;
    let t37179 = 0.984817913114256917e-7_f64 * t35051;
    let t37180 = 0.15716995342493974597e-7_f64 * t35056;
    let t37181 = 0.42206481990611010728e-7_f64 * t35059;
    let t37182 = 0.99044544404633838508e-5_f64 * t35062;
    let t37184 = 0.26519114751114692796e-6_f64 * t35069;
    let t37185 = 0.42206481990611010728e-7_f64 * t35071;
    let t37186 = 0.2698871527777777778e-4_f64 * t35074;
    (t37172, t37174, t37175, t37177, t37178, t37179, t37180, t37181, t37182, t37184, t37185, t37186)
}
