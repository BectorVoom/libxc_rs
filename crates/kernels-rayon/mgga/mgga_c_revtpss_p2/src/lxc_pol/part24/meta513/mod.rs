//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta513 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1530;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1531;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta513(t23862: f64, t3172: f64, t4837: f64, t1041: f64, t23822: f64, t11710: f64, t23920: f64, t3091: f64, t1058: f64, t23961: f64, t11859: f64, t11922: f64, t24008: f64, t23820: f64, t73: f64, t1063: f64, t23485: f64, t247: f64, t3109: f64, t23993: f64, t3115: f64, t23935: f64, t4899: f64, t15932: f64, t19826: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t79107, t79112, t79139, t79141, t79155) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1530(t23862, t3172, t4837, t1041, t23822, t11710, t23920, t3091, t1058, t23961, t11859, t11922, t24008);
        let (t79159, t79219, t79233, t79253, t79290) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1531(t23820, t73, t1063, t23485, t247, t3109, t11922, t23993, t3115, t23935, t4899, t15932, t19826);
    (t79107, t79112, t79139, t79141, t79155, t79159, t79219, t79233, t79253, t79290)
}
