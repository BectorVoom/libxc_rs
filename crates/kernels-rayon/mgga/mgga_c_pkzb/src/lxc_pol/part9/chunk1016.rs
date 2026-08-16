//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1016/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1016(t2382: f64, t6416: f64, t8254: f64, t824: f64, t919: f64, t2371: f64, t300: f64, t3236: f64, t2383: f64, t2185: f64, t3175: f64, t2888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8255 = t6416 * t2382;
    let t8256 = t8254 * t8255;
    let t8259 = t919 * t824;
    let t8260 = t2371 * t8259;
    let t8261 = t8254 * t8260;
    let t8264 = t300 * t3236;
    let t8265 = t8264 * t2383;
    let t8269 = t3175 * t2185;
    let t8270 = t2888 * t8269;
    (t8255, t8256, t8260, t8261, t8264, t8265, t8269, t8270)
}
