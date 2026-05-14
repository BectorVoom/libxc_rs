//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 10/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk10<F: Float>(t22: F, t21: F, t5: F, t11: F, t14: F, t17: F, t13: F) -> (F, F, F, F, F, F) {
    let t23 = 1.0 / t22;
    let t25 = t21 * t5 * t23;
    let t27 = 0.379785e1 * t14 + 0.8969e0 * t11 + 0.204775e0 * t17 + 0.123235e0 * t25;
    let t30 = 1.0 + 0.16081824322151104822e2 / t27;
    let t31 = f64::ln(t30);
    let t33 = 0.62182e-1 * t13 * t31;
    (t23, t25, t27, t30, t31, t33)
}
