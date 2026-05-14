//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1231/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1231<F: Float>(t2360: F, t7021: F, t2349: F, t2665: F, t446: F, t2682: F, t44280: F, t7036: F, t1882: F, t28797: F, t11176: F, t1485: F, t28757: F, t112807: F, t10409: F, t112812: F) -> (F, F, F, F, F, F, F) {
    let t113286 = t7021 * t2360;
    let t113289 = t446 * t2665 * t113286 * t2349;
    let t113293 = t446 * t44280 * t7036 * t2682;
    let t113295 = t1882 * t28797;
    let t113296 = 2.0 / 9.0 * t113295;
    let t113298 = t1485 * t11176 * t28757;
    let t113301 = t446 * t2665 * t112807;
    let t113304 = t446 * t10409 * t112812;
    (t113289, t113293, t113295, t113296, t113298, t113301, t113304)
}
