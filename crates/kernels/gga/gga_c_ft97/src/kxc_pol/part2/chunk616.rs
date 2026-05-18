//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 616/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk616<F: Float>(t230: F, t327: F, t3700: F, t18: F, t231: F, t893: F, t1270: F, t2253: F, t1268: F, t668: F, t2923: F, t505: F) -> (F, F, F, F, F, F) {
    let t4342 = t230 * t327;
    let t4343 = t4342 * t3700;
    let t4347 = t231 * t893 * t18;
    let t4350 = t2253 * t1270;
    let t4352 = t1268 * t668;
    let t4354 = t2923 * t4352 * t505;
    (t4342, t4343, t4347, t4350, t4352, t4354)
}
