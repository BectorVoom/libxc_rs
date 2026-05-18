//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 474/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk474<F: Float>(t3578: F, t609: F, t144: F, t1053: F, t2142: F, t2140: F, t2165: F, t2167: F, t28: F, t3480: F, t3485: F, t3489: F, t3541: F, t3545: F, t3548: F, t3551: F, t3567: F, t3571: F, t3575: F, t446: F, t89: F) -> (F, F, F, F, F) {
    let t3579 = t3578 * t609;
    let t3580 = t144 * t3579;
    let t3583 = t2142 * t1053;
    let t3584 = t144 * t3583;
    let t3587 = t2165 / F::new(9.0) + t2167 / F::new(9.0) - t2140 / F::new(9.0) + t446 * t3480 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t446 * t3485 - t3489 / F::new(9.0) + t89 * t28 * t3541 / F::new(3.0) + t3545 / F::new(9.0) - t446 * t3548 / F::new(3.0) + t3551 / F::new(9.0) - t446 * t3567 / F::new(3.0) - t446 * t3571 / F::new(3.0) - t446 * t3575 / F::new(3.0) - t446 * t3580 / F::new(3.0) - t446 * t3584 / F::new(3.0);
    (t3579, t3580, t3583, t3584, t3587)
}
