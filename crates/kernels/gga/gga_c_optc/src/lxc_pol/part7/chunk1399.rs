//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1399/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1399<F: Float>(t26160: F, t26163: F, t26168: F, t26170: F, t26173: F, t27346: F, t27843: F, t27846: F, t27849: F, t27856: F, t27858: F, t27860: F, t3245: F, t4281: F, t4290: F) -> F {
    let t27862 = -t26160 + t26163 + F::new(2.0) / F::new(9.0) * t27843 + t26168 + t26170 - F::new(4.0) / F::new(3.0) * t27846 + F::new(8.0) / F::new(9.0) * t27849 - t26173 + F::new(6.0) * t4281 * t3245 * t4290 * t27346 + F::new(56.0) / F::new(81.0) * t27856 + F::new(8.0) / F::new(9.0) * t27858 - F::new(2.0) / F::new(3.0) * t27860;
    t27862
}
