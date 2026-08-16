//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 947/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk947<F: Float>(t1457: F, t1572: F, t41869: F, t12900: F, t4950: F, t41774: F, t41778: F, t12897: F, t12922: F, t26939: F, t12926: F, t1641: F) -> (F, F, F, F, F, F, F) {
    let t42269 = t1572 * t1457 * t41869;
    let t42272 = F::cast_from(0.71500979903700853338e0_f64) * t4950 * t12900;
    let t42275 = F::cast_from(0.71500979903700853338e0_f64) * t1572 * t1457 * t41774;
    let t42278 = F::cast_from(0.71500979903700853338e0_f64) * t1572 * t1457 * t41778;
    let t42279 = t4950 * t12897;
    let t42282 = F::cast_from(0.42900587942220512003e1_f64) * t26939 * t12922;
    let t42284 = F::cast_from(0.46011511144704899612e1_f64) * t1641 * t12926;
    (t42269, t42272, t42275, t42278, t42279, t42282, t42284)
}
