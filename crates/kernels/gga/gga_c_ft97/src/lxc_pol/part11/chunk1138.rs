//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1138/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1138<F: Float>(t2: F, t43917: F, t10603: F, t2766: F, t2771: F, t41482: F, t4206: F, t42083: F, t42154: F, t43351: F, t43355: F, t43367: F, t43371: F, t43382: F, t43888: F, t43890: F, t43904: F, t43906: F, t43908: F, t43910: F, t43913: F, t462: F) -> F {
    let t43918 = t43917 * t2;
    let t43922 = -F::new(8.0) * t462 * t2766 * t42083 - F::new(2.0) / F::new(3.0) * t462 * t2766 * t42154 + F::new(4.0) / F::new(9.0) * t43888 - F::new(4.0) / F::new(3.0) * t43890 - F::new(4.0) * t462 * t10603 * t43367 - F::new(4.0) * t462 * t2771 * t43355 + F::new(8.0) * t462 * t2771 * t43371 - F::new(12.0) * t462 * t4206 * t41482 - F::new(8.0) / F::new(9.0) * t43904 + F::new(16.0) / F::new(9.0) * t43906 - F::new(8.0) / F::new(3.0) * t43908 + F::new(8.0) / F::new(3.0) * t43910 + F::new(8.0) * t462 * t43913 * t43382 - F::new(8.0) / F::new(3.0) * t462 * t43918 * t43351;
    t43922
}
