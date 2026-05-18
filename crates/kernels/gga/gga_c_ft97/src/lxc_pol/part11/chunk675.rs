//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 675/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk675<F: Float>(t9059: F, t9071: F, t9062: F, t9014: F, t9024: F, t9028: F, t9032: F, t9057: F, t9076: F, t9080: F, t9170: F, t9245: F, t9255: F) -> F {
    let t9380 = F::new(2.0) / F::new(9.0) * t9059;
    let t9383 = F::new(28.0) / F::new(81.0) * t9071;
    let t9390 = F::new(2.0) / F::new(9.0) * t9062;
    let t9393 = -t9380 - F::new(2.0) / F::new(3.0) * t9076 - F::new(2.0) / F::new(3.0) * t9080 - t9383 - t9014 / F::new(9.0) - t9170 / F::new(4.0) + F::new(2.0) * t9024 - F::new(10.0) / F::new(81.0) * t9028 - F::new(2.0) / F::new(3.0) * t9032 + F::new(4.0) / F::new(9.0) * t9057 - t9390 + t9245 / F::new(6.0) + t9255 / F::new(8.0);
    t9393
}
