//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 615/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk615<F: Float>(t12362: F, t12571: F, t157: F, t526: F, t1045: F, t2101: F, t2178: F, t358: F, t1055: F, t8232: F, t1030: F, t167: F) -> (F, F, F, F, F, F, F, F) {
    let t13119 = F::new(4.0) / F::new(27.0) * t12362;
    let t13123 = F::new(4.0) / F::new(9.0) * t12571;
    let t13140 = t526 * t157;
    let t13153 = t2101 * t1045;
    let t13165 = t2178 * t358;
    let t13187 = t8232 * t1055;
    let t13201 = t8232 * t1030;
    let t13208 = t2101 * t167;
    (t13119, t13123, t13140, t13153, t13165, t13187, t13201, t13208)
}
