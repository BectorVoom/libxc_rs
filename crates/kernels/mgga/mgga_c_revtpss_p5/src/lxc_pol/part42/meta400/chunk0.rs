//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1359/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1359<F: Float>(t1248: F, t6573: F, t1250: F, t3720: F, t19666: F, t5302: F, t1042: F, t17550: F, t19661: F, t1715: F, t17500: F, t5056: F, t5277: F) -> (F, F, F, F, F, F) {
    let t20856 = t6573 * t1248;
    let t20857 = t20856 * t1250;
    let t20858 = t3720 * t20857;
    let t20863 = t5302 * t19666;
    let t20864 = t1042 * t20863;
    let t20867 = t17550 * t19661;
    let t20868 = t1042 * t20867;
    let t20875 = t17500 * t1715;
    let t20876 = t1042 * t20875;
    let t20879 = t5277 * t5056;
    (t20856, t20858, t20864, t20868, t20876, t20879)
}
