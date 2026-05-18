//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 439/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk439<F: Float>(t238: F, t224: F, t2381: F, t2384: F, t2387: F, t2389: F, t2396: F, t2419: F, t2422: F, t2429: F, t2456: F, t678: F) -> F {
    let t239 = F::new(0.1e-59) < t238;
    let t2459 = piecewise3::<f64>(t239, F::new(0.67598802253579164263e-4) * t2381 * t2384 + F::new(0.23254900946437792e-1) * t2387 * t2389 + F::new(0.23254900946437792e-2) * t678 * t2396 - F::new(0.11627450473218896e-1) * t678 * t2419 + F::new(0.19365723406274399941e-3) * t678 * t2422 + F::new(2.0) * t224 * t2429 - t224 * t2456, F::new(0.0));
    t2459
}
