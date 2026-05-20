//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta960 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3221;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta960<F: Float>(t18299: F, t750: F, t49911: F, t4537: F, t18298: F, t705: F, t707: F, t14749: F, t14767: F, t198: F, t207: F, t2411: F, t39483: F, t39520: F, t39528: F, t39531: F, t39534: F, t39537: F, t4541: F, t4546: F, t18281: F, t706: F, t39737: F, t190: F, t60754: F, t18838: F, t892: F, t11075: F, t14375: F, t18435: F, t2403: F, t2404: F, t39540: F, t39741: F, t39744: F, t39747: F, t39750: F, t39756: F, t5962: F, t775: F) -> (F, F, F, F, F, F, F, F) {
        let (t61115, t61116, t61124, t61125) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3221::<F>(t18299, t750, t49911, t4537, t18298, t705, t707, t14749, t14767, t198, t207, t2411, t39483, t39520, t39528, t39531, t39534, t39537, t4541, t4546);
        let (t61131, t61135, t61138, t61146) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3222::<F>(t18281, t706, t750, t39737, t190, t60754, t18838, t892, t11075, t14375, t18435, t198, t2403, t2404, t39540, t39741, t39744, t39747, t39750, t39756, t4541, t5962, t775);
    (t61115, t61116, t61124, t61125, t61131, t61135, t61138, t61146)
}
