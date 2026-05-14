//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 589/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk589<F: Float>(t8210: F, t699: F, t798: F, t903: F, t2211: F, t4048: F, t739: F, t7903: F, t7906: F, t7912: F, t7915: F, t333: F, t698: F, t352: F, t321: F, t7818: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8211 = 0.39914139006212695214e-1 * t8210;
    let t8212 = t699 * t798;
    let t8213 = t903 * t8212;
    let t8214 = 0.35922725105591425692e0 * t8213;
    let t8215 = t2211 * t4048;
    let t8216 = t739 * t8215;
    let t8217 = 0.23948483403727617128e0 * t8216;
    let t8219 = 0.5107751987195740728e-4 * t7903;
    let t8220 = 0.2553875993597870364e-4 * t7906;
    let t8223 = 0.1702583995731913576e-4 * t7912;
    let t8224 = 0.10215503974391481456e-3 * t7915;
    let t8231 = t698 * t333;
    let t8232 = t8231 * t352;
    let t8235 = t698 * t321;
    let t8236 = t8235 * t352;
    let t8242 = 0.2927036860455597649e0 * t7818;
    (t8211, t8212, t8214, t8215, t8217, t8219, t8220, t8223, t8224, t8232, t8235, t8236, t8242)
}
