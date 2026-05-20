//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk670;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk671;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk672;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk673;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta145<F: Float>(t1398: F, t543: F, t550: F, t3992: F, t2661: F, t1384: F, t544: F, t235: F, t239: F, t820: F, t531: F, t549: F, t240: F, t72: F, t1386: F, t2482: F, t27: F, t136: F, t1389: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3994, t3995, t3996, t3999, t4000) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk670::<F>(t1398, t543, t550, t3992, t2661, t1384, t544, t235);
        let (t4002, t4003) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk671::<F>(t239, t4000, t820, t543);
        let (t4010, t4011, t4012, t4018) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk672::<F>(t531, t549, t240, t72, t1386, t2482, t27);
        let t4019 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk673::<F>(t136, t1389);
    (t3994, t3995, t3996, t3999, t4000, t4002, t4003, t4010, t4011, t4012, t4018, t4019)
}
