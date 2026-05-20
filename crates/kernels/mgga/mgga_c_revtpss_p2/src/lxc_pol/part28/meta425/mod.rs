//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta425 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1600;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1601;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1602;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta425<F: Float>(t12160: F, t4891: F, t1043: F, t4772: F, t1045: F, t3117: F, t1086: F, t4746: F, t3090: F, t15822: F, t3160: F, t1065: F, t2852: F, t1469: F, t2251: F, t1042: F, t3173: F, t4879: F, t1063: F, t11802: F, t11814: F, t11818: F, t11994: F, t3115: F, t3120: F, t3164: F, t3188: F, t4803: F, t4808: F, t4825: F, t4902: F, t4186: F, t999: F, t4872: F, t4866: F, t73: F, t3095: F, t3092: F, t2857: F, t357: F, t4781: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15917, t15920, t15922, t15925, t15926, t15932, t15935) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1600::<F>(t12160, t4891, t1043, t4772, t1045, t3117, t1086, t4746, t3090, t15822, t3160, t1065, t2852);
        let t15936 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1601::<F>(t1469, t2251);
        let (t15938, t15949) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1602::<F>(t15935, t15936, t1042, t3173, t4879, t1063, t11802, t11814, t11818, t11994, t15917, t15922, t15926, t15932, t3115, t3120, t3164, t3188, t4803, t4808, t4825, t4902);
        let (t15952, t15957, t15959, t15964) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1603::<F>(t4186, t999, t4872, t1042, t4866, t73, t3095, t3092, t2857, t357, t2251, t4781);
    (t15920, t15922, t15925, t15936, t15938, t15949, t15952, t15957, t15959, t15964)
}
