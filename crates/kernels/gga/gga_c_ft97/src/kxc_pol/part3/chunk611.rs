//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 611/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk611<F: Float>(t241: F, t258: F, t5132: F, t2518: F, t3688: F, t3710: F, t4920: F, t4924: F, t4928: F, t4932: F, t4937: F, t5056: F, t5094: F, t5122: F) -> (F, F) {
    let t5134 = t241 * t5132 * t258;
    let t5147 = -t5094 / F::new(4.0) + t5122 / F::new(2.0) + t2518 + F::new(2.0) / F::new(9.0) * t3688 + F::new(2.0) / F::new(3.0) * t3710 - F::new(2.0) / F::new(9.0) * t4920 + F::new(2.0) / F::new(3.0) * t4924 + F::new(2.0) / F::new(3.0) * t4928 - t4932 / F::new(3.0) + F::new(2.0) * t4937 - t5056;
    (t5134, t5147)
}
