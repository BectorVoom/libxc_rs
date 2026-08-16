//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta213 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk858;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk859;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk860;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta213<F: Float>(t4757: F, t996: F, t1096: F, t1651: F, t1079: F, t2848: F, t3070: F, t4571: F, t4576: F, t4581: F, t4585: F, t1678: F, t994: F, t1668: F, t73: F, t3095: F, t3092: F, t3093: F, t357: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t4758, t4764, t4772) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk858::<F>(t4757, t996, t1096, t1651, t1079, t2848, t3070, t4571, t4576, t4581, t4585);
        let (t4773, t4778, t4781) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk859::<F>(t4772, t996, t1678, t994, t1668, t73);
        let (t4782, t4783, t4786) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk860::<F>(t3095, t4781, t3092, t3093, t357);
    (t4758, t4764, t4772, t4773, t4778, t4781, t4782, t4783, t4786)
}
