//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 861/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk861<F: Float>(t10036: F, t10079: F, t1310: F, t16346: F, t16347: F, t16348: F, t4744: F, t6488: F, t6492: F, t6823: F, t6827: F, t6840: F) -> F {
    let t16619 = t6488 - t6823 + t6827 + t16346 - t16347 - t16348 - F::new(7.0) * t10036 + F::new(3.0) / F::new(2.0) * t10079 + t6492 - t6840 + F::new(3.0) / F::new(2.0) * t1310 * t4744;
    t16619
}
