//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 562/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk562<F: Float>(t4431: F, t464: F, t463: F, t1800: F, t24: F, t4436: F, t4495: F, t469: F, t1773: F, t3125: F, t3144: F, t4512: F, t4515: F, t4519: F, t462: F, t92: F) -> (F, F, F, F, F) {
    let t4522 = t464 * t4431;
    let t4523 = t463 * t4522;
    let t4527 = t24 * t1800 * t4436;
    let t4531 = t24 * t469 * t4495;
    let t4533 = t1773 + F::new(2.0) / F::new(9.0) * t3125 + F::new(2.0) / F::new(3.0) * t3144 - F::new(2.0) / F::new(9.0) * t462 * t4512 + F::new(2.0) / F::new(3.0) * t462 * t4515 + F::new(2.0) / F::new(3.0) * t462 * t4519 - t462 * t4523 / F::new(3.0) + F::new(2.0) * t92 * t4527 - t92 * t4531;
    (t4522, t4523, t4527, t4531, t4533)
}
