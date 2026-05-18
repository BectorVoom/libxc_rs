//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 563/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk563<F: Float>(t4533: F, t457: F, t91: F, t1832: F, t2981: F, t3006: F, t4420: F, t4424: F, t4428: F, t4434: F, t4439: F, t4498: F, t4507: F) -> (F, F) {
    let t4535 = t91 * t457 * t4533;
    let t4545 = -t4507 / F::new(12.0) + t4535 / F::new(6.0) + t1832 + F::new(2.0) / F::new(27.0) * t2981 + F::new(2.0) / F::new(9.0) * t3006 - F::new(2.0) / F::new(27.0) * t4420 + F::new(2.0) / F::new(9.0) * t4424 + F::new(2.0) / F::new(9.0) * t4428 - t4434 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t4439 - t4498 / F::new(3.0);
    (t4535, t4545)
}
