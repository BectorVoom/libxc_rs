//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 911/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk911<F: Float>(t4768: F, t8282: F, t1771: F, t4776: F, t4772: F, t62246: F, t62287: F, t62309: F, t62317: F, t4743: F, t8232: F, t4819: F) -> (F, F, F, F, F, F, F, F, F) {
    let t62669 = t8282 * t4768;
    let t62745 = t1771 * t4776;
    let t62751 = t1771 * t4772;
    let t62822 = F::new(4.0) / F::new(9.0) * t62246;
    let t62846 = F::new(4.0) / F::new(27.0) * t62287;
    let t62853 = F::new(8.0) / F::new(81.0) * t62309;
    let t62856 = F::new(8.0) / F::new(27.0) * t62317;
    let t63120 = t8232 * t4743;
    let t63157 = t8232 * t4819;
    (t62669, t62745, t62751, t62822, t62846, t62853, t62856, t63120, t63157)
}
