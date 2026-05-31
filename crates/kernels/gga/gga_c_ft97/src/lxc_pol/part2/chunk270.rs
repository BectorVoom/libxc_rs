//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 270/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk270<F: Float>(t1017: F, t526: F, t27: F, t89: F, t1000: F, t518: F, t515: F, t167: F, t569: F, t925: F, t574: F, t582: F, t958: F) -> (F, F, F, F, F, F, F) {
    let t1018 = t526 * t1017;
    let t1020 = t89 * t27 * t1018;
    let t1022 = -t518 - t1000 / F::cast_from(18.0_f64) - t1020 / F::cast_from(6.0_f64);
    let t1023 = t515 * t1022;
    let t1026 = t569 * t167 * t925;
    let t1030 = t574 * t167 * t1017;
    let t1033 = t582 * t958;
    (t1018, t1020, t1022, t1023, t1026, t1030, t1033)
}
