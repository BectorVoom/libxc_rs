//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1060/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1060<F: Float>(t140: F, t86867: F, t4431: F, t4822: F, t11761: F, t12791: F, t17338: F, t24: F, t4714: F, t49921: F, t586: F, t62745: F, t62751: F, t78179: F, t78181: F, t78183: F, t78185: F, t78188: F, t78242: F, t78247: F, t78249: F, t92: F) -> (F, F, F) {
    let t141 = F::new(0.1e-59) < t140;
    let t86868 = piecewise3::<f64>(t141, t86867, F::new(0.0));
    let t86876 = t4822 * t4431;
    let t86891 = -t92 * t24 * t586 * t86868 - F::new(8.0) * t11761 * t17338 * t4822 * t4714 - F::new(8.0) * t11761 * t12791 * t86876 + F::new(4.0) / F::new(9.0) * t78179 - F::new(16.0) / F::new(9.0) * t78181 + F::new(8.0) / F::new(3.0) * t78183 - F::new(8.0) * t78185 + F::new(40.0) / F::new(81.0) * t78188 + F::new(112.0) / F::new(27.0) * t49921 - F::new(8.0) / F::new(3.0) * t62745 + F::new(16.0) / F::new(3.0) * t62751 - F::new(8.0) / F::new(3.0) * t78242 + F::new(8.0) / F::new(9.0) * t78247 - F::new(4.0) / F::new(3.0) * t78249;
    (t86868, t86876, t86891)
}
