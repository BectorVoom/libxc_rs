//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2187;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2188;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2189;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2190;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta479(t3151: f64, t3154: f64, t15907: f64, t3117: f64, t11795: f64, t11859: f64, t11866: f64, t11875: f64, t15859: f64, t15862: f64, t15865: f64, t15866: f64, t15888: f64, t15892: f64, t15895: f64, t15899: f64, t15906: f64, t3184: f64, t375: f64, t4834: f64, t4912: f64, t12160: f64, t4891: f64, t1043: f64, t4772: f64, t1045: f64, t1086: f64, t4746: f64, t3090: f64, t15822: f64, t3160: f64, t1065: f64, t2852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15908, t15909, t15910, t15913) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2187(t3151, t3154, t15907, t3117, t11795, t11859, t11866, t11875, t15859, t15862, t15865, t15866, t15888, t15892, t15895, t15899, t15906, t3184, t375, t4834, t4912);
        let t15917 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2188(t12160, t4891);
        let (t15920, t15921, t15922, t15925, t15926) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2189(t1043, t4772, t1045, t3117, t1086, t4746, t3090);
        let (t15932, t15935) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2190(t15822, t3160, t1065, t2852);
    (t15908, t15909, t15910, t15913, t15917, t15920, t15921, t15922, t15925, t15926, t15932, t15935)
}
