//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1446;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1447;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1448;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta402<F: Float>(t15135: F, t2908: F, t141: F, t11341: F, t15140: F, t15145: F, t930: F, t15149: F, t1593: F, t2435: F, t4584: F, t689: F, t13312: F, t905: F, t904: F, t128: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15178, t15181, t15184, t15187, t15189) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1446::<F>(t15135, t2908, t141, t11341, t15140, t15145, t930, t15149, t1593, t2435);
        let t15191 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1447::<F>(t4584, t689);
        let (t15192, t15193, t15195) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1448::<F>(t15191, t13312, t905, t904, t128);
    (t15178, t15181, t15184, t15187, t15189, t15191, t15192, t15193, t15195)
}
