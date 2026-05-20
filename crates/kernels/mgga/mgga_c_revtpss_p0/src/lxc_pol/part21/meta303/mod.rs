//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1556;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1557;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta303<F: Float>(t10654: F, t2782: F, t2760: F, t822: F, t2718: F, t860: F, t2722: F, t836: F, t231: F, t243: F, t816: F, t9707: F, t813: F, t2394: F, t2476: F, t236: F, t807: F, t2689: F, t2694: F, t2430: F, t854: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10655, t10657, t10661, t10665) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1556::<F>(t10654, t2782, t2760, t822, t2718, t860, t2722, t836);
        let t10666 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1557::<F>(t10665, t231);
        let (t10673, t10674, t10675, t10676, t10678, t10680) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1558::<F>(t243, t816, t9707, t813, t2394, t2476, t236, t807, t2689, t2694, t2430, t854);
    (t10655, t10657, t10661, t10665, t10666, t10673, t10674, t10675, t10676, t10678, t10680)
}
