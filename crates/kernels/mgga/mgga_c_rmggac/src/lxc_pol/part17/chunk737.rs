//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 737/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk737<F: Float>(t5011: F, t511: F, t270: F, t38843: F, t7349: F, t7351: F, t2019: F, t2339: F, t7926: F, t118: F, t2001: F, t2318: F, t498: F, t1986: F, t495: F, t36343: F, t8457: F) -> (F, F, F, F, F, F) {
    let t40193 = t5011 * t511;
    let t40198 = t7349 * t7351 * t38843 * t270;
    let t40201 = t2019 * t7926 * t2339;
    let t40231 = t2001 * t118 * t2318 * t498;
    let t40246 = t1986 * t118 * t2318 * t495;
    let t40250 = t36343 * t8457;
    (t40193, t40198, t40201, t40231, t40246, t40250)
}
