//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 972/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk972<F: Float>(t2112: F, t30244: F, t1369: F, t28: F, t1359: F, t4778: F, t586: F, t5890: F, t1039: F, t6615: F, t30105: F, t526: F, t27: F, t89: F, t1017: F, t26791: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t30245 = t2112 * t30244;
    let t30247 = t1369 * t28 * t30245;
    let t30249 = t1359 * t4778;
    let t30250 = t586 * t30249;
    let t30252 = t5890 * t28 * t30250;
    let t30254 = t6615 * t1039;
    let t30255 = t586 * t30254;
    let t30257 = t5890 * t28 * t30255;
    let t30262 = t526 * t30105;
    let t30264 = t89 * t27 * t30262;
    let t30266 = t26791 * t1017;
    (t30245, t30247, t30249, t30250, t30252, t30254, t30255, t30257, t30262, t30264, t30266)
}
