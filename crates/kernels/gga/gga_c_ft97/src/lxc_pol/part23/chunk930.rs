//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 930/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk930<F: Float>(t245: F, t27921: F, t27961: F, t28009: F, t28472: F, t1459: F, t18: F, t1577: F, t21: F, t363: F, t5: F, t6200: F, t6953: F, t920: F, t375: F, t7087: F, t89: F) -> (F, F, F) {
    let t246 = 10000000.0 <= t245;
    let t28474 = t27921 + t27961 + t28009 + t28472;
    let t28484 = t1459 * t18;
    let t28489 = piecewise3(t246, 0.0, t5 * t28474 * t21 / 4.0 + t5 * t6953 * t363 / 4.0 + t5 * t6200 * t920 / 4.0 + t5 * t28484 * t1577 / 2.0);
    let t28491 = t89 * t375 * t7087;
    (t28474, t28489, t28491)
}
