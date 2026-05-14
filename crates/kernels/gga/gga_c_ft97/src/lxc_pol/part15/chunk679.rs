//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 679/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk679<F: Float>(t17016: F, t925: F, t2210: F, t167: F, t20035: F, t569: F, t1060: F, t4458: F, t20660: F, t9432: F, t12664: F, t4724: F, t144: F, t1053: F, t9439: F, t4805: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t20874 = t17016 * t925;
    let t20875 = t2210 * t20874;
    let t20880 = t569 * t167 * t20035;
    let t20884 = t569 * t1060 * t4458;
    let t20888 = t9432 * t167 * t20660;
    let t20893 = t12664 * t4724;
    let t20894 = t144 * t20893;
    let t20897 = t4724 * t1053;
    let t20898 = t9439 * t20897;
    let t20899 = t144 * t20898;
    let t20902 = t1053 * t4805;
    (t20874, t20875, t20880, t20884, t20888, t20893, t20894, t20897, t20898, t20899, t20902)
}
