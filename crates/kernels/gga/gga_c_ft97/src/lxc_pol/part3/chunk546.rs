//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 546/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk546<F: Float>(t1268: F, t2938: F, t898: F, t904: F, t2402: F, t2946: F, t3738: F, t3741: F, t3744: F, t3748: F, t4068: F, t4072: F, t688: F) -> (F, F, F) {
    let t4357 = t2938 * t1268;
    let t4359 = t898 * t4357 * t904;
    let t4370 = -F::new(0.117377e0) * t4068 * t688 + F::new(0.234754e0) * t4072 + t2946 + F::new(0.4814361111111111111e-1) * t2402 + F::new(0.4814361111111111111e-1) * t3738 - F::new(0.9628722222222222222e-1) * t3741 + F::new(0.28886166666666666666e0) * t3744 + F::new(0.28886166666666666666e0) * t3748;
    (t4357, t4359, t4370)
}
