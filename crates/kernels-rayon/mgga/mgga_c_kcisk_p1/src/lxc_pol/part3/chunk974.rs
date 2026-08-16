//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 974/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk974(t14187: f64, t492: f64, t4237: f64, t1483: f64, t4175: f64, t1501: f64, t4193: f64, t4200: f64, t4215: f64, t13328: f64, t484: f64, t13331: f64, t470: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14356 = t14187 * t492;
    let t14357 = t14356 * t4237;
    let t14359 = t1483 * t4175;
    let t14361 = t1501 * t4193;
    let t14363 = t4215 * t4200;
    let t14364 = t484 * t13328;
    let t14365 = t14364 * sigma0;
    let t14366 = t470 * t13331;
    (t14357, t14359, t14361, t14363, t14365, t14366)
}
