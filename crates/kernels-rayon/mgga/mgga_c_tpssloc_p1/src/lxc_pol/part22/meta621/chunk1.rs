//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2154/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2154(t53096: f64, t11647: f64, t1731: f64, t3577: f64, t44951: f64, t4949: f64, t3242: f64, t3448: f64, t11718: f64, t52835: f64, t11147: f64, t15394: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53097 = t53096 / 216.0_f64;
    let t53099 = t1731 * t11647;
    let t53161 = t3577 * t44951 * t4949;
    let t53162 = t53161 / 6912.0_f64;
    let t53187 = t3448 * t3242;
    let t53238 = t52835 * t11718;
    let t53249 = t15394 * t11147;
    (t53097, t53099, t53162, t53187, t53238, t53249)
}
