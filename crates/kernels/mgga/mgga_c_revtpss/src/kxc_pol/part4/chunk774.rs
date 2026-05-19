//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 774/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk774<F: Float>(t225: F, t4066: F, t1419: F, t213: F, t1425: F, t560: F, t1444: F, t1429: F, t2435: F, t1428: F, t2777: F, t2439: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4067 = t4066 * t225;
    let t4071 = t213 * t1419;
    let t4075 = F::new(1.0) / t1425 / t560;
    let t4076 = t225 * t4075;
    let t4077 = t1444 * t1444;
    let t4078 = t4076 * t4077;
    let t4082 = F::cast_from(0.73171657588172351096e-2_f64) * t2435 * t1429;
    let t4083 = t2777 * t1428;
    let t4085 = F::cast_from(0.65049603595885220126e-3_f64) * t2439 * t4083;
    (t4067, t4071, t4075, t4076, t4077, t4078, t4082, t4083, t4085)
}
