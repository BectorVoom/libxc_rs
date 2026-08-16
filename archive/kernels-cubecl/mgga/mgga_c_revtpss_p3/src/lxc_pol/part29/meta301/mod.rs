//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1192;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1193;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta301<F: Float>(t4018: F, t9970: F, t3930: F, t4059: F, t1386: F, t2482: F, t596: F, t4021: F, t1398: F, t1412: F, t3938: F, t3992: F, t2661: F, t1384: F, t235: F, t4003: F, t543: F, t27: F, t4000: F, t221: F, t4004: F, t4019: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9971, t9973, t9976, t9977, t9980, t9981) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1192::<F>(t4018, t9970, t3930, t4059, t1386, t2482, t596, t4021, t1398, t1412, t3938, t3992);
        let (t9982, t9990, t9991, t9994, t10001, t10003) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1193::<F>(t2661, t9981, t1384, t235, t4003, t543, t2482, t27, t4000, t221, t4004, t4019);
    (t9971, t9973, t9976, t9977, t9980, t9982, t9990, t9991, t9994, t10001, t10003)
}
