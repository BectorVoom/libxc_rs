//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1013/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1013<F: Float>(t96926: F, t96940: F, t96951: F, t96955: F, t96968: F, t96975: F, t96985: F, t97003: F, t97022: F, t97029: F, t97046: F, t97084: F, t97089: F, t97092: F, t97144: F, t97154: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t97320 = t96926 / 18.0;
    let t97324 = t96940 / 9.0;
    let t97327 = t96951 / 6.0;
    let t97329 = 2.0 / 27.0 * t96955;
    let t97333 = 2.0 / 9.0 * t96968;
    let t97335 = t96975 / 3.0;
    let t97339 = t96985 / 9.0;
    let t97344 = t97003 / 27.0;
    let t97350 = t97022 / 18.0;
    let t97352 = 14.0 / 81.0 * t97029;
    let t97356 = 2.0 * t97046;
    let t97367 = t97084 / 3.0;
    let t97369 = 2.0 / 3.0 * t97089;
    let t97370 = 4.0 / 3.0 * t97092;
    let t97381 = t97144 / 3.0;
    let t97384 = 2.0 / 3.0 * t97154;
    (t97320, t97324, t97327, t97329, t97333, t97335, t97339, t97344, t97350, t97352, t97356, t97367, t97369, t97370, t97381, t97384)
}
