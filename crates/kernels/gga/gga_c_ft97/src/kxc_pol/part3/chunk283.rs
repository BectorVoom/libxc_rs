//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 283/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk283<F: Float>(t1096: F, t680: F, t203: F, t222: F, t205: F, t207: F, rho1: F) -> (F, F, F, F) {
    let t1097 = t680 * t1096;
    let t1100 = t203 * t222;
    let t1101 = t205 * rho1;
    let t1103 = F::new(1.0) / t207 / t1101;
    (t1097, t1100, t1101, t1103)
}
