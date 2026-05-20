//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1444/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1444<F: Float>(t1222: F, t17472: F, t1012: F, t13026: F, t1263: F, t5245: F, t1234: F, t5390: F, t3704: F, t5293: F, t3172: F, t5286: F) -> (F, F, F, F, F, F) {
    let t17474 = t1222 * t17472 / F::new(324.0);
    let t17475 = t1012 * t13026;
    let t17500 = t1263 * t5245;
    let t17505 = t1234 * t5390;
    let t17509 = F::cast_from(0.15244095330869239812e-2_f64) * t5293 * t3704;
    let t17544 = t3172 * t5286;
    (t17474, t17475, t17500, t17505, t17509, t17544)
}
