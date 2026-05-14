//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 807/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk807<F: Float>(t9497: F, t9839: F, t500: F, t5606: F, t1415: F, t2275: F, t1340: F, t2279: F, t9834: F, t9837: F) -> (F, F, F, F, F) {
    let t9840 = t9497 * t9839;
    let t9842 = t5606 * t500;
    let t9844 = t1415 * t2275;
    let t9846 = t1340 * t2279;
    let t9848 = t9834 / 16.0 - t9837 / 16.0 + t9840 / 24.0 - t9842 / 128.0 + t9844 / 128.0 - t9846 / 96.0;
    (t9840, t9842, t9844, t9846, t9848)
}
