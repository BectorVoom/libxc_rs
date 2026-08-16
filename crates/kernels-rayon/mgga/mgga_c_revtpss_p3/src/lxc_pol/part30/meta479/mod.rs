//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1805;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta479(t114: f64, t25821: f64, t624: f64, t655: f64, t665: f64, t2339: f64, t68: f64, t2340: f64, t2366: f64, t6998: f64, t1312: f64, t7235: f64, t7313: f64, t2322: f64, t7003: f64, t18163: f64, t1937: f64, t4254: f64, t6993: f64, t7239: f64, t508: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25822, t25823, t25824, t25826, t25832) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1805(t114, t25821, t624, t655, t665, t2339, t68, t2340, t2366, t6998);
        let (t25834, t25838, t25840, t25842, t25844, t25846, t25851) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1806(t1312, t25832, t7235, t7313, t2322, t7003, t18163, t1937, t4254, t6993, t7239, t508);
    (t25822, t25823, t25824, t25826, t25832, t25834, t25838, t25840, t25842, t25844, t25846, t25851)
}
