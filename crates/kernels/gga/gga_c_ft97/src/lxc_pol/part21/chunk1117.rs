//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1117/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1117<F: Float>(t1286: F, t29735: F, t376: F, t29602: F, t8466: F, t101943: F, t101949: F, t101959: F, t101961: F, t102018: F, t1308: F, t16065: F, t16547: F, t22907: F, t22908: F, t25523: F, t25528: F, t25558: F, t25564: F, t25577: F, t25584: F, t28: F, t6423: F, t93946: F) -> (F, F) {
    let t115269 = t1286 * t376 * t29735;
    let t115271 = t8466 * t29602;
    let t115273 = 4.0 / 9.0 * t25577 * t22907 * t22908 * t16065 - t101943 + t101949 + t101959 + 2.0 / 27.0 * t101961 - 2.0 / 3.0 * t25584 * t6423 + t1286 * t28 * t1308 * t16547 / 6.0 + 2.0 * t25558 * t25564 + t102018 - 2.0 / 3.0 * t1286 * t28 * t25528 * t25523 + t115269 / 9.0 + 4.0 * t115271 - t93946;
    (t115271, t115273)
}
