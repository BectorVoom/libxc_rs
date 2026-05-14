//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 648/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk648<F: Float>(t79: F, t11126: F, t11223: F, t11330: F, t11389: F, t370: F, t27: F, t89: F, t1904: F, t2992: F, t1564: F, t446: F, t11174: F, t17: F, t355: F, t3001: F, t1755: F, t3013: F) -> (F, F, F, F, F, F, F, F) {
    let t80 = 0.1e-59 < t79;
    let t11392 = piecewise3(t80, t11126 + t11223 + t11330 + t11389, 0.0);
    let t11393 = t370 * t11392;
    let t11395 = t89 * t27 * t11393;
    let t11397 = t2992 * t1904;
    let t11398 = t1564 * t11397;
    let t11399 = t446 * t11398;
    let t11401 = t11174 * t17;
    let t11402 = t11401 * t355;
    let t11404 = t89 * t11402 * t3001;
    let t11406 = t3013 * t1755;
    (t11392, t11395, t11397, t11399, t11401, t11402, t11404, t11406)
}
