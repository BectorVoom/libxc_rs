//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1110/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1110<F: Float>(t26982: F, t8392: F, t1882: F, t27281: F, t26978: F, t27191: F, t604: F, t27313: F, t27193: F, t6692: F, t8232: F, t2178: F, t6718: F, t1359: F, t9438: F, t27001: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t107370 = 4.0 / 9.0 * t8392 * t26982;
    let t107379 = 2.0 / 27.0 * t1882 * t27281;
    let t107381 = 2.0 / 9.0 * t1882 * t26978;
    let t107399 = t604 * t27191;
    let t107412 = 2.0 / 9.0 * t1882 * t27313;
    let t107417 = 2.0 / 9.0 * t1882 * t27193;
    let t107418 = t8232 * t6692;
    let t107420 = t2178 * t6718;
    let t107448 = t9438 * t1359;
    let t107470 = 4.0 / 3.0 * t8392 * t27001;
    (t107370, t107379, t107381, t107399, t107412, t107417, t107418, t107420, t107448, t107470)
}
