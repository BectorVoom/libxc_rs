//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3197/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3197(t3625: f64, t44250: f64, t5401: f64, t127: f64, t5277: f64, t12866: f64, t3630: f64, t17550: f64, t372: f64, t17352: f64, t3153: f64, t3623: f64, t53667: f64) -> (f64, f64, f64, f64, f64) {
    let t58889 = t3625 * t44250 * t5401;
    let t58895 = t127 * t5277;
    let t58897 = t12866 * t58895 * t3630;
    let t58899 = t372 * t17550;
    let t58909 = t372 * t17352 * t3153;
    let t58919 = t3623 * t53667;
    (t58889, t58897, t58899, t58909, t58919)
}
