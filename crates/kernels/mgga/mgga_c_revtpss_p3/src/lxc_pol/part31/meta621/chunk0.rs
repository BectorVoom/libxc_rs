//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2071/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2071<F: Float>(t7058: F, t99321: F, t7759: F, t822: F, t25310: F, t27279: F, t27186: F, t93321: F, t93374: F, t122: F, t72: F, t2466: F) -> (F, F, F, F, F, F, F) {
    let t99323 = F::cast_from(0.14456046980341999104e-1_f64) * t7058 * t99321;
    let t99334 = t822 * t7759;
    let t99342 = F::cast_from(0.14456046980341999104e-1_f64) * t25310 * t27279;
    let t99344 = F::cast_from(0.14456046980341999104e-1_f64) * t93321 * t27186;
    let t99346 = F::cast_from(0.25702851531048074406e-1_f64) * t93374 * t27186;
    let t99348 = t7759 * t72 * t122;
    let t99349 = t99348 * t2466;
    (t99323, t99334, t99342, t99344, t99346, t99348, t99349)
}
