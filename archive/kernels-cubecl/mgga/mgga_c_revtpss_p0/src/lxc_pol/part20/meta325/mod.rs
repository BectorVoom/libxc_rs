//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta325 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1238;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1239;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1240;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta325<F: Float>(t12987: F, t480: F, t12629: F, t482: F, t371: F, t372: F, t127: F, t3672: F, t3671: F, t140: F, t3693: F, t1222: F, t10326: F, t1225: F, t1012: F, t1235: F, t1238: F, t1261: F, t12933: F, t12938: F, t12942: F, t12945: F, t12949: F, t12953: F, t12956: F, t12960: F, t12964: F, t12967: F, t12972: F, t12976: F, t12979: F, t12985: F, t3663: F, t3667: F, t3674: F, t3711: F, t3714: F, t1224: F, t3362: F, t10356: F, t1226: F, t697: F, t3688: F, t3700: F, t12268: F, t3698: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12988, t12989, t12991, t12995, t12996, t12999) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1238::<F>(t12987, t480, t12629, t482, t371, t372, t127, t3672, t3671, t140, t3693, t1222);
        let (t13001, t13005) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1239::<F>(t10326, t1225, t1012, t1222, t1235, t1238, t1261, t12933, t12938, t12942, t12945, t12949, t12953, t12956, t12960, t12964, t12967, t12972, t12976, t12979, t12985, t12988, t12991, t12996, t12999, t3663, t3667, t3674, t3711, t3714);
        let (t13007, t13008, t13012, t13015, t13018, t13020) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1240::<F>(t1224, t3362, t10356, t1012, t1226, t697, t1222, t140, t3688, t3700, t12268, t3698);
    (t12988, t12989, t12991, t12995, t13001, t13005, t13007, t13008, t13012, t13015, t13018, t13020)
}
