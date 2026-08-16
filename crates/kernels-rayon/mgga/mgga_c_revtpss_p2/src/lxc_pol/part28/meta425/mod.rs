//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1600;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1601;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1602;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta425(t12160: f64, t4891: f64, t1043: f64, t4772: f64, t1045: f64, t3117: f64, t1086: f64, t4746: f64, t3090: f64, t15822: f64, t3160: f64, t1065: f64, t2852: f64, t1469: f64, t2251: f64, t1042: f64, t3173: f64, t4879: f64, t1063: f64, t11802: f64, t11814: f64, t11818: f64, t11994: f64, t3115: f64, t3120: f64, t3164: f64, t3188: f64, t4803: f64, t4808: f64, t4825: f64, t4902: f64, t4186: f64, t999: f64, t4872: f64, t4866: f64, t73: f64, t3095: f64, t3092: f64, t2857: f64, t357: f64, t4781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15917, t15920, t15922, t15925, t15926, t15932, t15935) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1600(t12160, t4891, t1043, t4772, t1045, t3117, t1086, t4746, t3090, t15822, t3160, t1065, t2852);
        let t15936 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1601(t1469, t2251);
        let (t15938, t15949) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1602(t15935, t15936, t1042, t3173, t4879, t1063, t11802, t11814, t11818, t11994, t15917, t15922, t15926, t15932, t3115, t3120, t3164, t3188, t4803, t4808, t4825, t4902);
        let (t15952, t15957, t15959, t15964) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1603(t4186, t999, t4872, t1042, t4866, t73, t3095, t3092, t2857, t357, t2251, t4781);
    (t15920, t15922, t15925, t15936, t15938, t15949, t15952, t15957, t15959, t15964)
}
