//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1315;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1316;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1317;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta373(t15885: f64, t341: f64, t225: f64, t366: f64, t1058: f64, t4794: f64, t1651: f64, t3151: f64, t3155: f64, t3117: f64, t3162: f64, t11243: f64, t72: f64, t3088: f64, t12078: f64, t11249: f64, t1668: f64, t3154: f64, t11795: f64, t11859: f64, t11866: f64, t11875: f64, t15859: f64, t15862: f64, t15865: f64, t15866: f64, t3184: f64, t375: f64, t4834: f64, t4912: f64, t12160: f64, t4891: f64, t1043: f64, t4772: f64, t1045: f64, t1086: f64, t4746: f64, t3090: f64, t15822: f64, t3160: f64, t1065: f64, t2852: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15886, t15888, t15892, t15893, t15895, t15899, t15904) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1315(t15885, t341, t225, t366, t1058, t4794, t1651, t3151, t3155, t3117, t3162, t11243, t72);
        let (t15905, t15906, t15907) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1316(t15904, t3088, t12078, t11249, t1668);
        let t15913 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1317(t3151, t3154, t15907, t3117, t11795, t11859, t11866, t11875, t15859, t15862, t15865, t15866, t15888, t15892, t15895, t15899, t15906, t3184, t375, t4834, t4912);
        let (t15917, t15920, t15922, t15926, t15932, t15935) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1318(t12160, t4891, t1043, t4772, t1045, t3117, t1086, t4746, t3090, t15822, t3160, t1065, t2852);
    (t15886, t15893, t15904, t15905, t15907, t15913, t15917, t15920, t15922, t15926, t15932, t15935)
}
