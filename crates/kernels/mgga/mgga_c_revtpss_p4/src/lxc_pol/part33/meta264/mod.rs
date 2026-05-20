//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta264 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta264<F: Float>(t532: F, t7311: F, t1450: F, t2014: F, t1448: F, t4147: F, t2034: F, t1459: F, t2042: F, t116: F, t1936: F) -> (F, F, F, F, F, F, F, F) {
        let (t7312, t7313, t7314, t7315, t7316, t7317, t7329, t7330) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1179::<F>(t532, t7311, t1450, t2014, t1448, t4147, t2034, t1459, t2042, t116, t1936);
    (t7312, t7313, t7314, t7315, t7316, t7317, t7329, t7330)
}
