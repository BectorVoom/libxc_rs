//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta385 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1276;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1277;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta385<F: Float>(t19501: F, t3095: F, t3092: F, t1043: F, t3155: F, t6271: F, t3117: F, t12131: F, t357: F, t4786: F, t6100: F, t1065: F, t6244: F, t906: F, t1042: F, t3172: F, t6301: F, t1041: F, t5819: F, t606: F) -> (F, F, F, F, F, F, F) {
        let (t19626, t19636, t19641, t19645, t19649) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1276::<F>(t19501, t3095, t3092, t1043, t3155, t6271, t3117, t12131, t357, t4786, t6100, t1065, t6244);
        let (t19651, t19659, t19661) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1277::<F>(t19649, t906, t1042, t3172, t6301, t1041, t5819, t606);
    (t19626, t19636, t19641, t19645, t19651, t19659, t19661)
}
