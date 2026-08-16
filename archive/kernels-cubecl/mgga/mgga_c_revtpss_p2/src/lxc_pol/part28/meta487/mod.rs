//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta487 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1850;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta487<F: Float>(t7235: F, t7313: F, t2322: F, t7003: F, t18163: F, t1937: F, t4254: F, t6993: F, t7239: F, t25832: F, t508: F, t651: F) -> (F, F, F, F, F, F, F) {
        let (t25838, t25840, t25842, t25844, t25846, t25851, t25853) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1850::<F>(t7235, t7313, t2322, t7003, t18163, t1937, t4254, t6993, t7239, t25832, t508, t651);
    (t25838, t25840, t25842, t25844, t25846, t25851, t25853)
}
