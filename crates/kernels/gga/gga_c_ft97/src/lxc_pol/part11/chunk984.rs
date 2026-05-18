//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 984/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk984<F: Float>(t1636: F, t2076: F, t89: F, t375: F, t9008: F, t9018: F, t1987: F, t40301: F, t40306: F, t40309: F, t40312: F, t40315: F, t40318: F, t40321: F, t40490: F, t40494: F, t40497: F, t40500: F) -> (F, F, F, F, F) {
    let t40503 = t89 * t1636 * t2076;
    let t40506 = t89 * t375 * t9008;
    let t40509 = t89 * t375 * t9018;
    let t40512 = t89 * t1636 * t1987;
    let t40514 = -F::new(16.0) / F::new(9.0) * t40301 + F::new(8.0) * t40306 - F::new(8.0) * t40309 - F::new(8.0) / F::new(9.0) * t40312 - F::new(16.0) / F::new(27.0) * t40315 + F::new(4.0) / F::new(9.0) * t40318 + F::new(40.0) / F::new(81.0) * t40321 + t40490 / F::new(2.0) - F::new(3.0) / F::new(4.0) * t40494 + F::new(112.0) / F::new(81.0) * t40497 + F::new(16.0) / F::new(9.0) * t40500 - F::new(8.0) / F::new(3.0) * t40503 + F::new(4.0) / F::new(3.0) * t40506 + F::new(8.0) * t40509 + F::new(16.0) / F::new(3.0) * t40512;
    (t40503, t40506, t40509, t40512, t40514)
}
