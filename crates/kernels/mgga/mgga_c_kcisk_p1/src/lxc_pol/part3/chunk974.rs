//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 974/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk974<F: Float>(t14187: F, t492: F, t4237: F, t1483: F, t4175: F, t1501: F, t4193: F, t4200: F, t4215: F, t13328: F, t484: F, t13331: F, t470: F, sigma0: F) -> (F, F, F, F, F, F) {
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
