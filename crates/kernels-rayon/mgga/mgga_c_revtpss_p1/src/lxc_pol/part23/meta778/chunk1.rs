//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2583/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2583(t58850: f64, t17361: f64, t3708: f64, t3625: f64, t44250: f64, t5401: f64, t127: f64, t5277: f64, t17550: f64, t372: f64, t3623: f64, t53667: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58851 = 0.14291339372689912324e-3_f64 * t58850;
    let t58882 = t3708 * t17361;
    let t58883 = 0.14291339372689912324e-3_f64 * t58882;
    let t58889 = t3625 * t44250 * t5401;
    let t58895 = t127 * t5277;
    let t58899 = t372 * t17550;
    let t58919 = t3623 * t53667;
    (t58851, t58883, t58889, t58895, t58899, t58919)
}
