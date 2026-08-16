//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2524;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta617<F: Float>(t19705: F, t4873: F, t3092: F, t357: F, t4866: F, t4893: F, t3117: F, t19450: F, t4900: F, t11661: F, t19501: F, t1047: F, t1063: F, t12013: F, t16067: F, t16089: F, t19688: F, t19693: F, t19697: F, t19702: F, t3127: F, t4803: F, t4808: F, t4834: F, t4892: F, t4899: F, t6308: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19706, t19707, t19716, t19717, t19718, t19721, t19722, t19725, t19726, t19729) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2524::<F>(t19705, t4873, t3092, t357, t4866, t4893, t3117, t19450, t4900, t11661, t19501, t1047, t1063, t12013, t16067, t16089, t19688, t19693, t19697, t19702, t3127, t4803, t4808, t4834, t4892, t4899, t6308);
    (t19706, t19707, t19716, t19717, t19718, t19721, t19722, t19725, t19726, t19729)
}
