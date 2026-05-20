//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1997;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1998;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta579<F: Float>(t25310: F, t25331: F, t2435: F, t25339: F, t11064: F, t7086: F, t25604: F, t995: F, t357: F, t988: F, t355: F, t1071: F, t11239: F, t1078: F, t1982: F, t25610: F, t3093: F, t4975: F, t3058: F, t8521: F, t3143: F, t7135: F, t11865: F, t25516: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t93384, t93391, t93404, t93436, t93438, t93488) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1997::<F>(t25310, t25331, t2435, t25339, t11064, t7086, t25604, t995, t357, t988, t355, t1071, t11239);
        let (t93490, t93497, t93498, t93502, t93516, t93543) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1998::<F>(t1078, t1982, t93488, t25604, t25610, t3093, t4975, t3058, t8521, t3143, t7135, t11865, t25516);
    (t93384, t93391, t93404, t93436, t93438, t93490, t93497, t93498, t93502, t93516, t93543)
}
