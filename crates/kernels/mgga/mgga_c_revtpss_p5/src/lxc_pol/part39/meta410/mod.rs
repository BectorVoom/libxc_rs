//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1487;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta410<F: Float>(t4292: F, t648: F, t13514: F, t94: F, t1513: F, t2340: F, t4287: F, t665: F, t2366: F, t93: F, t31087: F, t575: F) -> (F, F, F, F, F, F, F) {
        let (t98487, t98535, t101457, t101460, t101463, t101522, t116890) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1487::<F>(t4292, t648, t13514, t94, t1513, t2340, t4287, t665, t2366, t93, t31087, t575);
    (t98487, t98535, t101457, t101460, t101463, t101522, t116890)
}
