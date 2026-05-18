//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 930/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk930<F: Float>(t1882: F, t8399: F, t8402: F, t104: F, t38061: F, t89: F, t8207: F, t8392: F, t8220: F, t487: F, t7800: F, t11810: F, t12020: F, t1559: F, t1580: F, t1588: F, t1643: F, t1755: F, t1876: F, t1901: F, t1902: F, t1903: F, t3187: F, t3193: F, t38937: F, t38947: F, t8217: F, t8510: F, t8518: F) -> F {
    let t39311 = t1882 * t8399;
    let t39313 = t1882 * t8402;
    let t39317 = F::new(280.0) / F::new(243.0) * t89 * t38061 * t104;
    let t39323 = t8392 * t8207;
    let t39329 = t8392 * t8220;
    let t39345 = t487 * t7800;
    let t39350 = F::new(4.0) / F::new(3.0) * t39311 + F::new(4.0) / F::new(9.0) * t39313 + t39317 - F::new(4.0) / F::new(3.0) * t1901 * t8217 * t1903 * t1580 * t1588 + F::new(8.0) / F::new(9.0) * t39323 + F::new(4.0) / F::new(9.0) * t1901 * t3193 * t8510 * t1643 + F::new(8.0) / F::new(9.0) * t39329 - F::new(8.0) * t1901 * t11810 * t487 * t1755 * t1876 - F::new(4.0) / F::new(3.0) * t1901 * t1902 * t3187 * t1559 * t1755 - F::new(16.0) / F::new(9.0) * t1901 * t8518 * t12020 * t38947 - F::new(16.0) / F::new(9.0) * t1901 * t3193 * t39345 * t38937;
    t39350
}
