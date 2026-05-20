//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1919;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta470<F: Float>(t15935: F, t19661: F, t1042: F, t19666: F, t4801: F, t1592: F, t16138: F, t19399: F, t247: F, t3116: F, t18942: F, t4915: F, t1011: F, t1063: F, t11656: F, t11994: F, t11999: F, t16057: F, t16062: F, t16064: F, t3127: F, t4837: F, t6263: F, t6312: F) -> (F, F, F, F, F, F, F, F) {
        let (t19929, t19930, t19933, t19934, t19939, t19940, t19944, t19947) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1919::<F>(t15935, t19661, t1042, t19666, t4801, t1592, t16138, t19399, t247, t3116, t18942, t4915);
        let t19950 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1920::<F>(t1011, t1063, t11656, t11994, t11999, t16057, t16062, t16064, t19930, t19934, t19940, t19944, t19947, t3127, t4837, t6263, t6312);
    (t19929, t19930, t19933, t19934, t19939, t19940, t19944, t19950)
}
