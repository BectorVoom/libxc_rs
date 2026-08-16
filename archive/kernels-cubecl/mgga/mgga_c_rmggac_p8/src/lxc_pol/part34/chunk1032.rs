//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1032/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1032<F: Float>(t76197: F, t76199: F, t5259: F, t551: F, t71949: F, t76201: F, t76203: F, t3203: F, t558: F, t69158: F, t69162: F, t69164: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77883 = F::cast_from(0.17961362552795712846e0_f64) * t76197;
    let t77884 = F::cast_from(0.44903406381989282115e-1_f64) * t76199;
    let t77886 = t5259 * t71949 * t551;
    let t77887 = F::cast_from(0.79828278012425390427e-1_f64) * t77886;
    let t77888 = F::cast_from(0.14967802127329760705e-1_f64) * t76201;
    let t77889 = F::cast_from(0.44903406381989282115e-1_f64) * t76203;
    let t77890 = t3203 * t558;
    let t77894 = t3203 * t551;
    let t77898 = F::cast_from(0.54549323308490683461e-1_f64) * t69158;
    let t77899 = F::cast_from(0.72732431077987577948e-1_f64) * t69162;
    let t77900 = F::cast_from(0.36366215538993788974e-1_f64) * t69164;
    (t77883, t77884, t77887, t77888, t77889, t77890, t77894, t77898, t77899, t77900)
}
