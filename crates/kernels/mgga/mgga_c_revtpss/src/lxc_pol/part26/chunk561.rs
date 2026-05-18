//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 561/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk561<F: Float>(t3356: F, t406: F, t3391: F, t1139: F, t3399: F, t281: F, t2902: F, t414: F, t1146: F, t698: F, t1224: F, t240: F) -> (F, F, F, F, F, F, F, F) {
    let t3402 = F::new(0.39862222222222222223e0) * t3356;
    let t3407 = F::new(1.0)/f64::sqrt(t406);
    let t3408 = t3407 * t3391;
    let t3410 = t1139 * t3399;
    let t3413 = t281 * t2902 * t414;
    let t3414 = F::new(0.13692777777777777778e0) * t3413;
    let t3415 = t698 * t1146;
    let t3417 = t240 * t1224;
    (t3402, t3407, t3408, t3410, t3413, t3414, t3415, t3417)
}
