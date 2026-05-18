//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 715/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk715<F: Float>(t1384: F, t9438: F, t3483: F, t27334: F, t23616: F, t23629: F, t23650: F, t27028: F, t27032: F, t27037: F, t27041: F, t27045: F, t27049: F, t27051: F, t27055: F, t27060: F) -> (F, F, F, F) {
    let t27335 = t9438 * t1384;
    let t27336 = t27335 * t3483;
    let t27337 = t27334 * t27336;
    let t27351 = t27028 / F::new(18.0) + t27032 / F::new(9.0) + t27037 / F::new(9.0) - F::new(2.0) * t27041 + F::new(2.0) / F::new(9.0) * t27045 - t27049 / F::new(6.0) - t27051 / F::new(27.0) + t27055 / F::new(3.0) - t23616 / F::new(36.0) - t23629 / F::new(9.0) - t27060 - t23650 / F::new(54.0);
    (t27335, t27336, t27337, t27351)
}
