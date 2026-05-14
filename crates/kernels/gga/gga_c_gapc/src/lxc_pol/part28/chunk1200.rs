//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1200/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1200<F: Float>(t35045: F, t35048: F, t35051: F, t35056: F, t35059: F, t35062: F, t35034: F, t35041: F, t37172: F, t37174: F, t37175: F, t35069: F, t35071: F, t35074: F, t35080: F, t35083: F) -> (F, F, F, F, F, F) {
    let t37177 = 0.38673709012042260328e-8 * t35045;
    let t37178 = 0.80966145833333333338e-4 * t35048;
    let t37179 = 0.984817913114256917e-7 * t35051;
    let t37180 = 0.15716995342493974597e-7 * t35056;
    let t37181 = 0.42206481990611010728e-7 * t35059;
    let t37182 = 0.99044544404633838508e-5 * t35062;
    let t37183 = -t37172 - 0.44198524585191154658e-7 * t35034 - t37174 - t37175 - 0.57970906942607043474e-5 * t35041 - t37177 - t37178 - t37179 + t37180 + t37181 - t37182;
    let t37184 = 0.26519114751114692796e-6 * t35069;
    let t37185 = 0.42206481990611010728e-7 * t35071;
    let t37186 = 0.2698871527777777778e-4 * t35074;
    let t37188 = 0.40483072916666666668e-3 * t35080;
    let t37189 = 0.18310351929594268994e-5 * t35083;
    (t37183, t37184, t37185, t37186, t37188, t37189)
}
