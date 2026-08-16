//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta179 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk777;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk778;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk779;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta179<F: Float>(t1412: F, t72: F, t245: F, t125: F, t1398: F, t1353: F, t543: F, t159: F, t550: F, t216: F, t124: F, t3829: F, t800: F, t1376: F, t2689: F, t1413: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t3935, t3936) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk777::<F>(t1412, t72, t245);
        let (t3937, t3938) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk778::<F>(t125, t1398, t1353, t543);
        let (t3940, t3943, t3944, t3946, t3950, t3951) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk779::<F>(t3937, t3938, t3936, t159, t550, t216, t124, t3829, t800, t1376, t2689, t1353, t1413);
    (t3935, t3936, t3938, t3940, t3943, t3944, t3946, t3950, t3951)
}
