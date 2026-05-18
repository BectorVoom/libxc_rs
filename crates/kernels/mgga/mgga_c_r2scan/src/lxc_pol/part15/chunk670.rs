//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 670/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk670<F: Float>(t4784: F, t4877: F, t61: F, t41: F, t1419: F, t458: F, t1416: F, t425: F, t1415: F, t405: F, t89: F, t2098: F) -> (F, F, F, F, F, F) {
    let t4878 = t4784 + t4877;
    let t4879 = t61 * t4878;
    let t4880 = t41 * t4879;
    let t4881 = t1419 * t458;
    let t4883 = t1416 * t425;
    let t4885 = t405 * t1415;
    let t4886 = t4885 * t89;
    let t4888 = F::new(1.0) / t2098;
    (t4880, t4881, t4883, t4885, t4886, t4888)
}
