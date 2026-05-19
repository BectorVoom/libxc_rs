//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 411/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk411<F: Float>(t1640: F, t1645: F, t1649: F, t1653: F, t2008: F, t2011: F, t2289: F, t637: F, t639: F, t2251: F, t2254: F, t2256: F, t2261: F, t2265: F, t2268: F, t2273: F, t2277: F, t2284: F, t631: F) -> (F, F, F) {
    let t2294 = -F::new(0.117377e0) * t2008 + F::new(0.234754e0) * t2011 + t2289 + F::cast_from(0.9628722222222222222e-1_f64) * t1640 - F::cast_from(0.9628722222222222222e-1_f64) * t1645 + F::cast_from(0.28886166666666666666e0_f64) * t1649 - F::cast_from(0.14443083333333333333e0_f64) * t1653;
    let t2296 = t637 * t639 * t2294;
    let t2299 = -t2251 - F::new(2.0) / F::new(9.0) * t2254 - F::new(2.0) / F::new(3.0) * t2256 + t631 * t2261 / F::new(18.0) - F::new(2.0) / F::new(3.0) * t2265 * t2268 - t631 * t2273 / F::new(3.0) + t631 * t2277 / F::new(6.0) - F::new(3.0) / F::new(2.0) * t631 * t2284 + t631 * t2296 / F::new(2.0);
    (t2294, t2296, t2299)
}
