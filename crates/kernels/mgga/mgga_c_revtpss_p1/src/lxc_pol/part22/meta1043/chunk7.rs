//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3653/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3653<F: Float>(t68297: F, t68301: F, t68305: F, t68310: F, t68332: F, t68334: F, t68336: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F) -> F {
    let t69058 = F::cast_from(0.71233333333333333332e-1_f64) * t68297 + F::cast_from(0.35616666666666666666e-1_f64) * t68301 + F::cast_from(0.10685e0_f64) * t68305 - F::cast_from(0.52765432098765432099e-1_f64) * t68310 + F::cast_from(0.79148148148148148146e-2_f64) * t68332 + F::cast_from(0.15829629629629629629e-1_f64) * t68334 + F::cast_from(0.47488888888888888888e-1_f64) * t68336 + F::cast_from(0.19787037037037037037e-1_f64) * t68342 + F::cast_from(0.23744444444444444444e0_f64) * t68347 - F::cast_from(0.71233333333333333332e-1_f64) * t68350 - F::cast_from(0.42739999999999999999e0_f64) * t68353 - F::cast_from(0.23744444444444444444e-1_f64) * t68357 + F::cast_from(0.4274e0_f64) * t68360;
    t69058
}
