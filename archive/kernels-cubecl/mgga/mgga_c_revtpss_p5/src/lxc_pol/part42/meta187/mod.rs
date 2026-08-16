//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta187 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk771;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk772;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk773;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta187<F: Float>(t1121: F, t4186: F, t1120: F, t128: F, t3357: F, t3358: F, t5044: F, t5049: F, t5054: F, t422: F, t1130: F, t1719: F, t1151: F, t1733: F, t3379: F) -> (F, F, F, F, F, F, F, F) {
        let t5056 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk771::<F>(t1121, t4186);
        let (t5057, t5058) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk772::<F>(t1120, t5056, t128);
        let (t5060, t5062, t5063, t5065, t5067) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk773::<F>(t3357, t3358, t5044, t5049, t5054, t5058, t422, t1130, t1719, t1151, t1733, t3379);
    (t5056, t5057, t5058, t5060, t5062, t5063, t5065, t5067)
}
