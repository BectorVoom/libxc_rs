//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 566/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk566<F: Float>(t942: F, t979: F, t452: F, t488: F, t1812: F, t2981: F, t3006: F, t4420: F, t4424: F, t4428: F, t4434: F, t4439: F, t4498: F, t4507: F, t4535: F) -> (F, F, F) {
    let t4572 = t942 * t979;
    let t4574 = t452 * t488 * t4572;
    let t4589 = -t4507 / F::new(4.0) + t4535 / F::new(2.0) + t1812 + F::new(2.0) / F::new(9.0) * t2981 + F::new(2.0) / F::new(3.0) * t3006 - F::new(2.0) / F::new(9.0) * t4420 + F::new(2.0) / F::new(3.0) * t4424 + F::new(2.0) / F::new(3.0) * t4428 - t4434 / F::new(3.0) + F::new(2.0) * t4439 - t4498;
    (t4572, t4574, t4589)
}
