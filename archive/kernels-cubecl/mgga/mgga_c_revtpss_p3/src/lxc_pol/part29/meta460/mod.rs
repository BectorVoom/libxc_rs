//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta460 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1710;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1711;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1712;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta460<F: Float>(t26265: F, t3917: F, t25899: F, t26231: F, t72: F, t7531: F, t686: F, t7284: F, t7289: F, t136: F, t2102: F, t2457: F, t25944: F, t25950: F, t7515: F, t213: F, t7506: F, t1445: F, t2103: F, t25909: F, t26232: F, t26235: F, t26238: F, t26241: F, t26246: F, t26251: F, t26253: F, t26257: F, t26263: F, t4132: F, t7292: F, t7295: F, t7511: F, t7532: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26266, t26268, t26270, t26271) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1710::<F>(t26265, t3917, t25899, t26231, t72, t7531, t686);
        let (t26272, t26274, t26276, t26277) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1711::<F>(t26271, t7284, t7289, t136, t2102, t2457);
        let (t26279, t26280, t26282, t26291) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1712::<F>(t25944, t26277, t25950, t7515, t213, t7506, t1445, t2103, t25909, t26232, t26235, t26238, t26241, t26246, t26251, t26253, t26257, t26263, t26266, t26268, t26272, t26274, t4132, t7292, t7295, t7511, t7532);
    (t26266, t26268, t26270, t26271, t26272, t26274, t26276, t26277, t26279, t26280, t26282, t26291)
}
