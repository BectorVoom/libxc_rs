//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1020/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1020(t32135: f64, t45963: f64, t10301: f64, t32148: f64, t32141: f64, t45972: f64, t10309: f64, t116: f64, t32160: f64, t25081: f64, t8567: f64, t11064: f64, t8489: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t119465 = t45963 * t32135;
    let t119468 = t10301 * t32148;
    let t119471 = t10301 * t32135;
    let t119500 = t10301 * t32141;
    let t119503 = t45972 * t32135;
    let t119508 = t10309 * t32148;
    let t119535 = t32160 * t116;
    let t119578 = t8567 * t25081;
    let t119675 = t8489 * t11064;
    (t119465, t119468, t119471, t119500, t119503, t119508, t119535, t119578, t119675)
}
