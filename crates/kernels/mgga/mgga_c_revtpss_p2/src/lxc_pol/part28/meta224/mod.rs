//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta224 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1056;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1057;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1058;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1059;
use chunk4::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1060;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta224<F: Float>(t1716: F, t689: F, t1469: F, t3362: F, t606: F, t3360: F, t128: F, t3367: F, t1120: F, t1121: F, t4186: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5044 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1056::<F>(t1716, t689);
        let (t5046, t5047, t5048, t5049) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1057::<F>(t1469, t3362, t606, t3360, t128);
        let (t5051, t5052, t5053, t5054) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1058::<F>(t1469, t3367, t606, t1120, t128);
        let t5056 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1059::<F>(t1121, t4186);
        let (t5057, t5058) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1060::<F>(t1120, t5056, t128);
    (t5044, t5046, t5047, t5048, t5049, t5051, t5052, t5053, t5054, t5056, t5057, t5058)
}
