//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta232 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1051;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1052;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1053;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta232<F: Float>(t1651: F, t996: F, t1695: F, t1079: F, t3070: F, t4571: F, t6094: F, t6098: F, t6102: F, t1592: F, t4823: F, t1042: F, t1469: F, t3094: F, t4781: F, t3092: F, t1668: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t6244 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1051::<F>(t1651);
        let (t6245, t6251, t6258) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1052::<F>(t6244, t996, t1651, t1695, t1079, t3070, t4571, t6094, t6098, t6102);
        let (t6259, t6262, t6263, t6266, t6267, t6268, t6271) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1053::<F>(t6258, t996, t1592, t4823, t1042, t1469, t3094, t4781, t3092, t1651, t1668);
    (t6244, t6245, t6251, t6258, t6259, t6262, t6263, t6266, t6267, t6268, t6271)
}
