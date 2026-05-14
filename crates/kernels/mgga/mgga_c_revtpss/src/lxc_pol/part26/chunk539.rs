//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 539/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk539<F: Float>(t1134: F, t3390: F, t3356: F, t3358: F, t3365: F, t3370: F, t3374: F, t1132: F, t406: F, t1139: F, t281: F, t2902: F, t414: F, t1146: F, t698: F, t1224: F, t240: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3391 = t1134 * t1134;
    let t3392 = t3390 * t3391;
    let t3394 = 4.0 / 9.0 * t3356;
    let t3399 = t3394 - 2.0 / 9.0 * t3358 - 2.0 / 9.0 * t3365 + 2.0 / 3.0 * t3370 + t3374 / 3.0;
    let t3400 = t1132 * t3399;
    let t3402 = 0.39862222222222222223e0 * t3356;
    let t3407 = 1.0/f64::sqrt(t406);
    let t3408 = t3407 * t3391;
    let t3410 = t1139 * t3399;
    let t3413 = t281 * t2902 * t414;
    let t3414 = 0.13692777777777777778e0 * t3413;
    let t3415 = t698 * t1146;
    let t3417 = t240 * t1224;
    (t3391, t3392, t3399, t3400, t3402, t3407, t3408, t3410, t3413, t3414, t3415, t3417)
}
