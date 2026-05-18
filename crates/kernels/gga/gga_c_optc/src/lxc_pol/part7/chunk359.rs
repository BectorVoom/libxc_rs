//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 359/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk359<F: Float>(t1196: F, t1200: F, t1205: F, t485: F, t275: F, t176: F, t1107: F, t496: F, t492: F, t490: F, t487: F, t426: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t1207 = t1196 * t485 - t1200 * t1205;
    let t1208 = t1207 * t275;
    let t1210 = t176 * t1208 * sigma2;
    let t1213 = t1107 * t496;
    let t1214 = t492 * t1213;
    let t1216 = t490 * t1214 / F::new(6.0);
    let t1217 = t176 * t487;
    let t1218 = t275 * sigma2;
    let t1219 = t1218 * t426;
    (t1207, t1210, t1214, t1216, t1217, t1218, t1219)
}
