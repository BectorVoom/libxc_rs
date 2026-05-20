//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta278 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1021;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1022;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta278<F: Float>(t240: F, t9940: F, t72: F, t595: F, t66: F, t247: F, t550: F, t548: F, t4010: F, t245: F, t3829: F, t543: F, t1386: F, t820: F, t844: F, t3940: F, t221: F, t3924: F, t4019: F, t4018: F, t3930: F, t4059: F, t2482: F, t596: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9942, t9949, t9953, t9954, t9955, t9956) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1021::<F>(t240, t9940, t72, t595, t66, t247, t550, t548, t4010, t245, t3829, t543);
        let (t9962, t9963, t9971, t9973, t9976) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1022::<F>(t1386, t820, t844, t3940, t221, t3924, t4019, t4018, t3930, t4059, t2482, t596);
    (t9942, t9949, t9953, t9954, t9955, t9956, t9962, t9963, t9971, t9973, t9976)
}
