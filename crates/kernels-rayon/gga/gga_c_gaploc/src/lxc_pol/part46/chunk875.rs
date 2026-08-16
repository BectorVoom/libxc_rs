//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 875/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk875(t1645: f64, t2859: f64, t9152: f64, t3149: f64, t8063: f64, t2877: f64, t9487: f64, t1457: f64, t1572: f64, t41869: f64, t12900: f64, t4950: f64) -> (f64, f64, f64, f64, f64) {
    let t42263 = 0.10725146985555128001e1_f64 * t2859 * t1645 * t9152;
    let t42265 = 0.23833659967900284446e0_f64 * t3149 * t8063;
    let t42267 = 0.35750489951850426669e0_f64 * t9487 * t2877;
    let t42269 = t1572 * t1457 * t41869;
    let t42272 = 0.71500979903700853338e0_f64 * t4950 * t12900;
    (t42263, t42265, t42267, t42269, t42272)
}
