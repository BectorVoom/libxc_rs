//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 989/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk989(t5259: f64, t551: f64, t71949: f64, t76201: f64, t76203: f64, t69158: f64, t69162: f64, t69164: f64, t2447: f64, t36: f64, t321: f64, t333: f64, t4669: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t77886 = t5259 * t71949 * t551;
    let t77887 = 0.79828278012425390427e-1_f64 * t77886;
    let t77888 = 0.14967802127329760705e-1_f64 * t76201;
    let t77889 = 0.44903406381989282115e-1_f64 * t76203;
    let t77898 = 0.54549323308490683461e-1_f64 * t69158;
    let t77899 = 0.72732431077987577948e-1_f64 * t69162;
    let t77900 = 0.36366215538993788974e-1_f64 * t69164;
    let t77901 = t2447 * t36;
    let t77903 = t5259 * t77901 * t321;
    let t77904 = 0.2993560425465952141e-1_f64 * t77903;
    let t77906 = t4669 * t77901 * t333;
    (t77887, t77888, t77889, t77898, t77899, t77900, t77901, t77904, t77906)
}
