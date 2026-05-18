//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 397/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk397<F: Float>(t2120: F, t579: F, t91: F, t1956: F, t1959: F, t1962: F, t1967: F, t1972: F, t1977: F, t1981: F, t1989: F, t2078: F, t2089: F) -> (F, F) {
    let t2122 = t91 * t579 * t2120;
    let t2124 = F::new(4.0) / F::new(27.0) * t1956;
    let t2133 = -t2089 / F::new(12.0) + t2122 / F::new(6.0) + t2124 + F::new(2.0) / F::new(27.0) * t1959 + F::new(2.0) / F::new(9.0) * t1962 - F::new(2.0) / F::new(27.0) * t1967 + F::new(2.0) / F::new(9.0) * t1972 + F::new(2.0) / F::new(9.0) * t1977 - t1981 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t1989 - t2078 / F::new(3.0);
    (t2122, t2133)
}
