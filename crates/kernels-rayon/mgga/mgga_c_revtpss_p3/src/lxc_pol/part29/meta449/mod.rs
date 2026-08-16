//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta449 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1678;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1679;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1680;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta449(t624: f64, t655: f64, t665: f64, t2339: f64, t68: f64, t2340: f64, t2366: f64, t6998: f64, t1450: f64, t3829: f64, t555: f64, t7063: f64, t1032: f64, t4075: f64, t545: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25823, t25824, t25825, t25826, t25827, t25829, t25865, t25875) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1678(t624, t655, t665, t2339, t68, t2340, t2366, t6998, t1450, t3829, t555, t7063);
        let (t25876, t25877) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1679(t1032, t4075, t545);
        let t25878 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1680(t25875, t25877);
        let t25894 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1681(t555, t786);
    (t25823, t25824, t25825, t25826, t25827, t25829, t25865, t25875, t25876, t25877, t25878, t25894)
}
