//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 718/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk718<F: Float>(t11286: F, t11312: F, t409: F, t64: F, t1602: F, t939: F, t11084: F, t7906: F, t11089: F, t1631: F, t11146: F, t11225: F, t11232: F, t11233: F, t11241: F, t11246: F, t11247: F, t11251: F, t1604: F, t1605: F, t1625: F, t1751: F, t3076: F, t3077: F, t3101: F, t372: F, t399: F, t428: F, t6426: F, t7877: F, t7879: F) -> F {
    let t11313 = t11286 + t11312;
    let t11315 = t64 * t409 * t11313;
    let t11318 = t1602 * t939;
    let t11321 = t7906 * t11084;
    let t11324 = t1631 * t11089;
    let t11327 = t1631 * t11146;
    let t11330 = F::new(4.0) * t3076 * t11225 * t428 + F::new(2.0) * t3076 * t3077 * t1751 - F::new(0.46509801892875584e-2) * t11232 * t11233 * t1625 + F::new(0.46509801892875584e-1) * t7877 * t6426 * t7879 + F::new(0.93019603785751168e-2) * t11241 * t11233 * t1604 + F::new(0.77462893625097599763e-3) * t11246 * t11247 * t1604 - F::new(2.0) * t11251 - t11315 - F::new(0.11854761295685025975e-1) * t3101 * t399 + F::new(0.46509801892875584e-1) * t11318 * t1605 - F::new(0.11619434043764639964e-3) * t372 * t11321 + F::new(0.46509801892875584e-2) * t372 * t11324 + F::new(0.23254900946437792e-2) * t372 * t11327;
    t11330
}
