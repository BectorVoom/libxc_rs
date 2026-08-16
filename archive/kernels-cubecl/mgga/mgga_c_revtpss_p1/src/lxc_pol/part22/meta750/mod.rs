//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta750 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2823;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta750<F: Float>(t11408: F, t941: F, t2979: F, t2986: F, t11465: F, t960: F, t2935: F, t2967: F, t11509: F, t3006: F, t2866: F, t2873: F) -> (F, F, F, F, F, F) {
        let (t41779, t41785, t41788, t41799, t41813, t41880) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2823::<F>(t11408, t941, t2979, t2986, t11465, t960, t2935, t2967, t11509, t3006, t2866, t2873);
    (t41779, t41785, t41788, t41799, t41813, t41880)
}
