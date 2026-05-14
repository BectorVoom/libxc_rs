//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 985/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk985<F: Float>(t22887: F, t600: F, t1644: F, t8544: F, t1665: F, t16351: F, t2382: F, t6802: F, t6835: F, t10557: F, t8550: F, t4699: F, t8574: F, t1980: F, t8612: F, t6856: F, t7506: F) -> (F, F, F, F, F, F, F, F) {
    let t22888 = t22887 * t600;
    let t22891 = t8544 * t1644;
    let t22893 = 1.0 * t22891 * t1665;
    let t22895 = 2.0 * t16351 * t2382;
    let t22897 = 2.0 * t6802 * t6835;
    let t22899 = 2.0 * t10557 * t8550;
    let t22901 = 1.0 * t4699 * t8574;
    let t22908 = t8612 * t1980;
    let t22915 = t6856 * t7506;
    (t22888, t22893, t22895, t22897, t22899, t22901, t22908, t22915)
}
