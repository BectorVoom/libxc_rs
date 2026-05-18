//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 615/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk615<F: Float>(t100: F, t8326: F, t3194: F, t8205: F, t1822: F, t1882: F, t1901: F, t446: F, t8475: F, t8477: F, t8480: F, t8483: F, t8485: F, t8487: F, t8491: F, t8496: F, t8499: F, t8503: F, t8507: F, t8512: F, t8516: F) -> (F, F, F, F) {
    let t8518 = t8326 * t100;
    let t8519 = t3194 * t8205;
    let t8520 = t8518 * t8519;
    let t8523 = t1882 * t1822;
    let t8525 = -F::new(4.0) / F::new(9.0) * t8475 + t8477 / F::new(3.0) + F::new(2.0) * t446 * t8480 + F::new(2.0) / F::new(3.0) * t8483 - F::new(4.0) / F::new(9.0) * t8485 - F::new(2.0) / F::new(3.0) * t8487 + t1901 * t8491 / F::new(3.0) + t1901 * t8496 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t8499 + F::new(2.0) * t446 * t8503 + F::new(2.0) / F::new(3.0) * t1901 * t8507 + t1901 * t8512 / F::new(3.0) + F::new(4.0) / F::new(9.0) * t8516 + F::new(2.0) / F::new(9.0) * t1901 * t8520 + t8523 / F::new(3.0);
    (t8518, t8519, t8520, t8525)
}
