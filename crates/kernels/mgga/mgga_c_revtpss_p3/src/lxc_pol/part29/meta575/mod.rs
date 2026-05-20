//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta575 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta575<F: Float>(t1565: F, t93066: F, t25222: F, t4345: F, t4349: F, t93072: F, t14910: F, t25270: F, t14678: F, t14673: F, t92955: F, t14688: F) -> (F, F, F, F, F, F, F) {
        let (t99009, t99011, t99013, t99015, t99017, t99019, t99021) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1923::<F>(t1565, t93066, t25222, t4345, t4349, t93072, t14910, t25270, t14678, t14673, t92955, t14688);
    (t99009, t99011, t99013, t99015, t99017, t99019, t99021)
}
