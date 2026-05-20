//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1527/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1527<F: Float>(t3801: F, t6748: F, t1209: F, t6695: F, t460: F, t1214: F, t6587: F, t1211: F, t6744: F, t1277: F, t1294: F, t6573: F) -> (F, F, F, F, F, F, F) {
    let t20692 = t6748 * t3801;
    let t20697 = t1209 * t6695;
    let t20700 = t460 * t6695;
    let t20703 = t6587 * t1214;
    let t20704 = t1211 * t20703;
    let t20709 = t6744 * t1214;
    let t20710 = t1277 * t20709;
    let t20714 = t1277 * t6573 * t1294;
    (t20692, t20697, t20700, t20703, t20704, t20710, t20714)
}
