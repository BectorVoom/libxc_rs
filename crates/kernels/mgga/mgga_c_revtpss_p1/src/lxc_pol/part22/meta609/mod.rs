//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta609 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2509;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2510;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2511;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta609<F: Float>(t1043: F, t6258: F, t1089: F, t3153: F, t6305: F, t4982: F, t999: F, t1024: F, t1083: F, t1087: F, t11940: F, t12122: F, t12149: F, t16544: F, t16559: F, t16566: F, t19438: F, t19443: F, t19447: F, t19453: F, t19457: F, t19463: F, t19479: F, t19484: F, t19488: F, t19492: F, t3223: F, t3287: F, t4857: F, t4954: F, t4977: F, t4988: F, t4992: F, t4996: F, t5005: F, t6368: F, t4757: F, t5004: F, t3291: F, t6244: F, t1082: F, t19399: F, t4866: F, t4893: F, t1647: F, t4980: F, t1071: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t19497, t19498, t19501) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2509::<F>(t1043, t6258, t1089, t3153, t6305);
        let (t19503, t19508) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2510::<F>(t4982, t999, t19501, t1024, t1083, t1087, t11940, t12122, t12149, t16544, t16559, t16566, t19438, t19443, t19447, t19453, t19457, t19463, t19479, t19484, t19488, t19492, t19498, t3223, t3287, t4857, t4954, t4977, t4988, t4992, t4996, t5005, t6368);
        let (t19509, t19512, t19515, t19521, t19526, t19533) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2511::<F>(t4757, t5004, t3291, t6244, t1082, t19399, t4866, t4982, t4893, t1647, t4980, t1071, t6305);
    (t19497, t19498, t19501, t19503, t19508, t19509, t19512, t19515, t19521, t19526, t19533)
}
