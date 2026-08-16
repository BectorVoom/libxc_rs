//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 875/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk875(t1330: f64, t72: f64, t757: f64, t530: f64, t566: f64, t525: f64) -> (f64, f64, f64, f64) {
    let t3825 = t1330 * t72;
    let t3826 = t3825 * t757;
    let t3828 = t530 * t566;
    let t3833 = 1.0_f64 / t525;
    (t3825, t3826, t3828, t3833)
}
