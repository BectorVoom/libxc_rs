//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 997/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk997<F: Float>(t26302: F, t30451: F, t13523: F, t20292: F, t26138: F, t26150: F, t26159: F, t30288: F, t30292: F, t30296: F, t30300: F, t30303: F, t30306: F) -> (F, F) {
    let t30452 = t26302 * t30451;
    let t30465 = -t13523 - F::new(0.23744444444444444444e-1) * t20292 + F::new(0.11872222222222222222e-1) * t26138 - F::new(0.35616666666666666666e-1) * t26150 + F::new(0.17808333333333333333e-1) * t26159 - F::new(0.19787037037037037037e-1) * t30288 + F::new(0.71233333333333333332e-1) * t30292 - F::new(0.35616666666666666666e-1) * t30296 - F::new(0.10685e0) * t30300 + F::new(0.10685e0) * t30303 - F::new(0.17808333333333333333e-1) * t30306;
    (t30452, t30465)
}
