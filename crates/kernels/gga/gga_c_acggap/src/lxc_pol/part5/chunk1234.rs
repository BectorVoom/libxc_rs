//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1234/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1234<F: Float>(t13748: F, t13750: F, t13754: F, t13771: F, t13814: F, t16233: F, t16238: F, t16241: F, t16244: F, t16249: F, t16253: F, t16255: F, t16264: F, t16274: F) -> F {
    let t22590 = -F::new(4.0) / F::new(3.0) * t16233 - t16238 / F::new(2.0) + t16241 / F::new(3.0) + t16244 / F::new(6.0) - F::new(5.0) / F::new(3.0) * t16249 + F::new(56.0) / F::new(9.0) * t16253 + F::new(4.0) * t16255 - F::new(4.0) / F::new(3.0) * t16264 - F::new(8.0) / F::new(3.0) * t16274 - t13814 + F::new(140.0) / F::new(27.0) * t13748 + F::new(14.0) / F::new(9.0) * t13750 - F::new(7.0) / F::new(9.0) * t13754 - F::new(5.0) / F::new(3.0) * t13771;
    t22590
}
