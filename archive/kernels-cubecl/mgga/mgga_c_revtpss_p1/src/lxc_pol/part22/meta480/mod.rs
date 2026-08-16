//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta480 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2191;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2192;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta480<F: Float>(t1469: F, t2251: F, t15935: F, t1042: F, t3173: F, t4879: F, t1063: F, t11802: F, t11814: F, t11818: F, t11994: F, t15917: F, t15922: F, t15926: F, t15932: F, t3115: F, t3120: F, t3164: F, t3188: F, t4803: F, t4808: F, t4825: F, t4902: F, t4186: F, t999: F, t4872: F, t4866: F, t73: F) -> (F, F, F, F, F, F, F, F, F) {
        let t15936 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2191::<F>(t1469, t2251);
        let (t15937, t15938, t15942, t15949) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2192::<F>(t15935, t15936, t1042, t3173, t4879, t1063, t11802, t11814, t11818, t11994, t15917, t15922, t15926, t15932, t3115, t3120, t3164, t3188, t4803, t4808, t4825, t4902);
        let (t15950, t15951, t15952, t15957) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2193::<F>(t4186, t999, t4872, t1042, t4866, t73);
    (t15936, t15937, t15938, t15942, t15949, t15950, t15951, t15952, t15957)
}
