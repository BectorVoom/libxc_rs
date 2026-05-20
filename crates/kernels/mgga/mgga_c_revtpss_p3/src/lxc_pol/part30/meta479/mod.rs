//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1805;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta479<F: Float>(t114: F, t25821: F, t624: F, t655: F, t665: F, t2339: F, t68: F, t2340: F, t2366: F, t6998: F, t1312: F, t7235: F, t7313: F, t2322: F, t7003: F, t18163: F, t1937: F, t4254: F, t6993: F, t7239: F, t508: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t25822, t25823, t25824, t25826, t25832) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1805::<F>(t114, t25821, t624, t655, t665, t2339, t68, t2340, t2366, t6998);
        let (t25834, t25838, t25840, t25842, t25844, t25846, t25851) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1806::<F>(t1312, t25832, t7235, t7313, t2322, t7003, t18163, t1937, t4254, t6993, t7239, t508);
    (t25822, t25823, t25824, t25826, t25832, t25834, t25838, t25840, t25842, t25844, t25846, t25851)
}
