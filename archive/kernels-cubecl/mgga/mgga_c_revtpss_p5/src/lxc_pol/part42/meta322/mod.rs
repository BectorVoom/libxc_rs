//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1102;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta322<F: Float>(t5711: F, t786: F, t1364: F, t1357: F, t5775: F, t689: F, t2470: F, t5721: F, t3915: F, t1445: F, t5599: F, t2435: F, t5600: F, t1426: F, t1893: F, t3917: F, t136: F, t1903: F, t2457: F, t9674: F, t10175: F, t5722: F, t122: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t14084, t14087, t14091, t14096, t14097) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1102::<F>(t5711, t786, t1364, t1357, t5775, t689, t2470, t5721, t3915, t1445, t5599, t2435, t5600);
        let (t14100, t14102, t14105, t14108, t14109) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1103::<F>(t1426, t1893, t786, t3917, t136, t1903, t2457, t9674, t10175, t5722, t122, t5721);
    (t14084, t14087, t14091, t14096, t14097, t14100, t14102, t14105, t14108, t14109)
}
