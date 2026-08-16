//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2122;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2123;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta504<F: Float>(t1043: F, t4772: F, t1045: F, t3117: F, t1086: F, t4746: F, t3090: F, t15822: F, t3160: F, t1065: F, t2852: F, t1469: F, t2251: F, t1042: F, t3173: F, t4879: F, t1063: F, t11802: F, t11814: F, t11818: F, t11994: F, t15917: F, t3115: F, t3120: F, t3164: F, t3188: F, t4803: F, t4808: F, t4825: F, t4902: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t15920, t15921, t15922, t15925, t15926, t15932, t15935, t15936) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2122::<F>(t1043, t4772, t1045, t3117, t1086, t4746, t3090, t15822, t3160, t1065, t2852, t1469, t2251);
        let (t15937, t15938, t15949) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2123::<F>(t15935, t15936, t1042, t3173, t4879, t1063, t11802, t11814, t11818, t11994, t15917, t15922, t15926, t15932, t3115, t3120, t3164, t3188, t4803, t4808, t4825, t4902);
    (t15920, t15921, t15922, t15925, t15926, t15932, t15935, t15936, t15937, t15938, t15949)
}
