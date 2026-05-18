//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 852/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk852<F: Float>(t1286: F, t1526: F, t1527: F, t2: F, t32031: F, t32043: F, t342: F, t343: F, t34592: F, t34596: F, t34601: F, t34607: F, t6512: F, t6517: F, t7151: F, t7152: F) -> F {
    let t34612 = (-t34592 * t7152 / F::new(6.0) + t32031 + t1286 * t34596 / F::new(18.0) + t1286 * t6517 / F::new(3.0) - t7151 * t34601 / F::new(6.0) - t32043 - t1526 * t1527 * t6512 / F::new(12.0) - t342 * t343 * t34607 / F::new(4.0)) * t2;
    t34612
}
