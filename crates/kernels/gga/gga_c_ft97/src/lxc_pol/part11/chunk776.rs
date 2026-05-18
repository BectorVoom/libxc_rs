//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 776/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk776<F: Float>(t10504: F, t2881: F, t10440: F, t10444: F, t10448: F, t10453: F, t10458: F, t10461: F, t10463: F, t10467: F, t10471: F, t10475: F, t10482: F, t10488: F, t10495: F, t10500: F, t1901: F) -> (F, F) {
    let t10505 = t2881 * t10504;
    let t10508 = t1901 * t10440 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t1901 * t10444 + F::new(2.0) / F::new(3.0) * t1901 * t10448 - F::new(2.0) / F::new(3.0) * t1901 * t10453 + t1901 * t10458 / F::new(3.0) - F::new(2.0) / F::new(9.0) * t10461 - F::new(2.0) / F::new(9.0) * t10463 + t1901 * t10467 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t1901 * t10471 - F::new(2.0) / F::new(9.0) * t1901 * t10475 + F::new(2.0) / F::new(9.0) * t1901 * t10482 + F::new(2.0) / F::new(9.0) * t1901 * t10488 - F::new(2.0) / F::new(3.0) * t1901 * t10495 + t1901 * t10500 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1901 * t10505;
    (t10505, t10508)
}
