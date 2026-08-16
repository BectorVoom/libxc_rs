//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 834/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk834<F: Float>(t34384: F, t7243: F, t7238: F, t7239: F, t1800: F, t34370: F, t28: F, t5665: F, t32094: F, t7824: F, t925: F, t5674: F) -> (F, F, F, F, F, F) {
    let t34385 = t7243 * t34384;
    let t34387 = t7238 * t7239 * t34385;
    let t34389 = t1800 * t34370;
    let t34391 = t5665 * t28 * t34389;
    let t34394 = t7824 * t32094 * t925;
    let t34395 = t5674 * t34394;
    (t34385, t34387, t34389, t34391, t34394, t34395)
}
