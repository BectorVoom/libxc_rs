//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1190/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1190<F: Float>(t35031: F, t35036: F, t35039: F, t35045: F, t35048: F, t35051: F, t35056: F, t35059: F, t35062: F, t35069: F, t35071: F, t35074: F, t35080: F, t35083: F, t35090: F, t35093: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37172 = 0.40483072916666666669e-4 * t35031;
    let t37174 = 0.50680539737635041234e-3 * t35036;
    let t37175 = 0.25301920572916666668e-5 * t35039;
    let t37177 = 0.38673709012042260328e-8 * t35045;
    let t37178 = 0.80966145833333333338e-4 * t35048;
    let t37179 = 0.984817913114256917e-7 * t35051;
    let t37180 = 0.15716995342493974597e-7 * t35056;
    let t37181 = 0.42206481990611010728e-7 * t35059;
    let t37182 = 0.99044544404633838508e-5 * t35062;
    let t37184 = 0.26519114751114692796e-6 * t35069;
    let t37185 = 0.42206481990611010728e-7 * t35071;
    let t37186 = 0.2698871527777777778e-4 * t35074;
    let t37188 = 0.40483072916666666668e-3 * t35080;
    let t37189 = 0.18310351929594268994e-5 * t35083;
    let t37191 = 0.10298285674687440379e-5 * t35090;
    let t37192 = 0.15716995342493974597e-7 * t35093;
    (t37172, t37174, t37175, t37177, t37178, t37179, t37180, t37181, t37182, t37184, t37185, t37186, t37188, t37189, t37191, t37192)
}
