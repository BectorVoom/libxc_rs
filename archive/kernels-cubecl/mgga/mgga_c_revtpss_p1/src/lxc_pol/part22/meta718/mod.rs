//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta718 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2753;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2754;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2755;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2756;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2757;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta718<F: Float>(t10963: F, t9303: F, t10069: F, t10934: F, t10518: F, t10542: F, t10612: F, t2398: F, t2434: F, t2626: F, t2629: F, t676: F, t9425: F, t2567: F, t2576: F, t2582: F, t2577: F, t268: F, t9326: F, t215: F, t2581: F, t2585: F, t675: F, t9273: F, t9276: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t39724, t39726, t39731, t39737, t39739, t39741, t39742) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2753::<F>(t10963, t9303, t10069, t10934, t10518, t10542, t10612, t2398, t2434, t2626, t2629, t676, t9425);
        let (t39744, t39747) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2754::<F>(t2629, t39742, t2567, t2576, t2582);
        let t39750 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2755::<F>(t2577, t268, t9326);
        let t39756 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2756::<F>(t215, t2581, t2585, t268);
        let t39760 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2757::<F>(t268, t675, t9273, t9276);
    (t39724, t39726, t39731, t39737, t39739, t39741, t39742, t39744, t39747, t39750, t39756, t39760)
}
