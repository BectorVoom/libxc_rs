//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1225/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1225<F: Float>(t101957: F, t6418: F, t22914: F, t29590: F, t1286: F, t29730: F, t376: F, t104016: F, t104025: F, t104031: F, t116973: F, t117888: F, t118154: F, t118325: F, t1564: F, t25539: F, t25577: F, t25622: F, t26117: F, t29602: F, t3052: F, t492: F, t6414: F, t8418: F) -> (F,) {
    let t118463 = t101957 * t6418;
    let t118465 = t22914 * t29590;
    let t118481 = t1286 * t376 * t29730;
    let t118485 = t118463 / 27.0 + t118465 / 81.0 - t104016 - 2.0 * t118154 + t104025 - 2.0 / 9.0 * t25577 * t1564 * t26117 * t3052 + t104031 + t6414 * t25539 / 3.0 + t6414 * t25622 / 3.0 - 12.0 * t8418 * t29602 * t492 + 8.0 * t118325 + 2.0 / 9.0 * t118481 - 2.0 * t116973 - 2.0 * t117888;
    (t118485,)
}
