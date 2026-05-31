//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 325/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk325<F: Float>(t1637: F, t26: F, t380: F, t458: F, t1554: F, t17: F) -> (F, F, F, F) {
    let t1638 = t26 * t1637;
    let t1639 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1638;
    let t1640 = t458 * t380;
    let t1642 = t17 * t1554;
    (t1638, t1639, t1640, t1642)
}
