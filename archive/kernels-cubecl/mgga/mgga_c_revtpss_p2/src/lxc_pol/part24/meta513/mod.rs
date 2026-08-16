//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1530;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1531;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta513<F: Float>(t23862: F, t3172: F, t4837: F, t1041: F, t23822: F, t11710: F, t23920: F, t3091: F, t1058: F, t23961: F, t11859: F, t11922: F, t24008: F, t23820: F, t73: F, t1063: F, t23485: F, t247: F, t3109: F, t23993: F, t3115: F, t23935: F, t4899: F, t15932: F, t19826: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t79107, t79112, t79139, t79141, t79155) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1530::<F>(t23862, t3172, t4837, t1041, t23822, t11710, t23920, t3091, t1058, t23961, t11859, t11922, t24008);
        let (t79159, t79219, t79233, t79253, t79290) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1531::<F>(t23820, t73, t1063, t23485, t247, t3109, t11922, t23993, t3115, t23935, t4899, t15932, t19826);
    (t79107, t79112, t79139, t79141, t79155, t79159, t79219, t79233, t79253, t79290)
}
