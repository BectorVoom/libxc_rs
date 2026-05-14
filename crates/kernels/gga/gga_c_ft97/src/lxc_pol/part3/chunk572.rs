//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 572/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk572<F: Float>(t1234: F, t2755: F, t91: F, t2766: F, t5098: F, t2771: F, t5213: F, t5105: F, t848: F, t5109: F, t192: F, t2781: F, t5225: F, t5299: F, t852: F, t2761: F, t4197: F, t4213: F, t462: F, t92: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5337 = t1234 * t1234;
    let t5339 = t91 * t2755 * t5337;
    let t5343 = t2766 * t5098;
    let t5346 = t2771 * t5213;
    let t5349 = t848 * t5105;
    let t5352 = t848 * t5109;
    let t5356 = t192 * t2781 * t5225;
    let t5360 = t192 * t852 * t5299;
    let t5362 = t2761 + 2.0 / 9.0 * t4197 + 2.0 / 3.0 * t4213 - 2.0 / 9.0 * t462 * t5343 + 2.0 / 3.0 * t462 * t5346 + 2.0 / 3.0 * t462 * t5349 - t462 * t5352 / 3.0 + 2.0 * t92 * t5356 - t92 * t5360;
    (t5337, t5339, t5343, t5346, t5349, t5352, t5356, t5360, t5362)
}
