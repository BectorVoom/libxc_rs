//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta594 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2011;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2012;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta594<F: Float>(t94522: F, t2018: F, t9646: F, t9723: F, t26014: F, t2689: F, t3994: F, t7028: F, t9845: F, t25240: F, t3951: F, t3964: F, t2681: F, t7269: F, t820: F, t1416: F, t240: F, t25981: F, t25987: F, t9775: F, t2453: F, t4086: F, t64: F, t9795: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t94523, t94526, t94527, t94537, t94540) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2011::<F>(t94522, t2018, t9646, t9723, t26014, t2689, t3994, t7028, t9845, t25240, t3951, t3964);
        let (t94545, t94546, t94550, t94554, t94564, t94565) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2012::<F>(t2681, t7269, t820, t1416, t240, t25981, t25987, t9775, t2453, t4086, t64, t9795);
    (t94523, t94526, t94527, t94537, t94540, t94545, t94546, t94550, t94554, t94564, t94565)
}
