//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2949/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2949(t11620: f64, t1651: f64, t11710: f64, t15969: f64, t4892: f64, t1062: f64, t15655: f64, t11239: f64, t1647: f64, t11245: f64, t11255: f64, t11643: f64, t15707: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53683 = t1651 * t11620;
    let t53690 = t4892 * t11710 * t15969;
    let t53692 = t15655 * t1062;
    let t53703 = t1647 * t11239;
    let t53704 = t53703 * t11245;
    let t53707 = t53703 * t11255;
    let t53710 = t15707 * t11643;
    (t53683, t53690, t53692, t53703, t53704, t53707, t53710)
}
