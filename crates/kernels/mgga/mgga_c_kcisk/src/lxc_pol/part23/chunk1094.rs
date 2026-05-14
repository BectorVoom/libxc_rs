//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1094/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1094<F: Float>(t21516: F, t21549: F, t21606: F, t21640: F, t21679: F, t21915: F, t21930: F, t21959: F, t1597: F, t2306: F, t4346: F, t19936: F, t19939: F, t19942: F, t19945: F, t19948: F, t19953: F, t19956: F, t19959: F, t19962: F, t4351: F, t6426: F) -> (F, F, F, F) {
    let t21962 = t21516 + t21549 + t21606 + t21640 + t21679 + t21915 + t21930 + t21959;
    let t21963 = t21962 * t1597;
    let t21969 = t2306 * t4346;
    let t21983 = 0.74498e-1 * t21969 * t4351 - 0.61905925925925925925e-2 * t19936 + 0.23214722222222222222e-2 * t19939 + 0.11607361111111111111e-2 * t19942 - 0.17411041666666666666e-2 * t19945 - 0.25794135802469135802e-3 * t19948 + 0.193e0 * t6426 * t4351 - 0.34822083333333333332e-2 * t19953 - 0.17411041666666666666e-2 * t19956 + 0.11607361111111111111e-2 * t19959 + 0.19345601851851851852e-2 * t19962;
    (t21962, t21963, t21969, t21983)
}
