//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 886/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk886<F: Float>(t1337: F, t918: F, t1248: F, t13614: F, t2075: F, t20295: F, t20298: F, t398: F, t5814: F, t5601: F, t3979: F, t5676: F, t13603: F, t5671: F, t1311: F, t3117: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20310 = t918 * t1337;
    let t20373 = t1248 * t13614 * t2075;
    let t20381 = 4.0 / 27.0 * t20295;
    let t20382 = 4.0 / 9.0 * t20298;
    let t20404 = t5814 * t398;
    let t20406 = t1248 * t20404 * t5601;
    let t20437 = t1248 * t3979 * t5676;
    let t20438 = 0.44152e0 * t20437;
    let t20440 = t1248 * t13603 * t5671;
    let t20448 = t3117 * t1311;
    (t20310, t20373, t20381, t20382, t20406, t20437, t20438, t20440, t20448)
}
