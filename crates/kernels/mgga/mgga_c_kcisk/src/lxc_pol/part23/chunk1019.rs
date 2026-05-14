//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1019/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1019<F: Float>(t1248: F, t19132: F, t6074: F, t3979: F, t5676: F, t13603: F, t5671: F, t19123: F, t4065: F, t13607: F, t19109: F, t1311: F, t3117: F, t19119: F, t20295: F, t1249: F, t19127: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20433 = t1248 * t6074 * t19132;
    let t20437 = t1248 * t3979 * t5676;
    let t20438 = 0.44152e0 * t20437;
    let t20440 = t1248 * t13603 * t5671;
    let t20443 = t1248 * t4065 * t19123;
    let t20446 = t1248 * t13607 * t19109;
    let t20448 = t3117 * t1311;
    let t20450 = t1248 * t20448 * t19119;
    let t20454 = 0.13418888888888888889e0 * t20295;
    let t20461 = t1248 * t1249 * t19127;
    (t20433, t20437, t20438, t20440, t20443, t20446, t20450, t20454, t20461)
}
