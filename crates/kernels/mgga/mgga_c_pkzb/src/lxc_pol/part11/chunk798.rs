//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 798/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk798<F: Float>(t2593: F, t2639: F, t179: F, t3403: F, t5221: F, t1702: F, t3407: F, t3402: F, t568: F, t581: F, t1024: F, t2575: F, t3406: F, t1706: F, t2592: F, t5225: F, t5265: F, t6873: F, t6885: F, t6894: F, t6914: F, t6928: F, t6933: F) -> (F, F, F, F, F, F, F, F) {
    let t8920 = t2593 * t2639;
    let t8921 = t179 * t8920;
    let t8924 = t5221 * t3403;
    let t8926 = t1702 * t3407;
    let t8931 = t581 * t3402 * t568;
    let t8935 = t581 * t1024 * t2575;
    let t8939 = t581 * t3406 * t568;
    let t8944 = 0.85748036236139473944e-3 * t2592 * t8921 - 7.0 / 48.0 * t8924 + 7.0 / 144.0 * t8926 - 0.80031500487063509016e-2 * t6873 - 0.80031500487063509015e-2 * t6885 - t6894 - t5225 * t8931 / 4.0 + t1706 * t8935 / 8.0 + t1706 * t8939 / 16.0 - 35.0 / 216.0 * t5265 - t6914 + t6928 - 35.0 / 108.0 * t6933;
    (t8920, t8921, t8924, t8926, t8931, t8935, t8939, t8944)
}
