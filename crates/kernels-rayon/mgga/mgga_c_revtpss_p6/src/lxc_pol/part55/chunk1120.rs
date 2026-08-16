//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1120/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1120(t10867: f64, t2061: f64, t11064: f64, t8019: f64, t116: f64, t29421: f64, t1468: f64, t1711: f64, t7897: f64, t8995: f64, t3999: f64, t8085: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t103452 = t10867 * t2061;
    let t103586 = t8019 * t11064;
    let t104115 = t29421 * t116;
    let t106589 = t11064 * t1468;
    let t107923 = t11064 * t1711;
    let t109269 = t7897 * t8995;
    let t109731 = t3999 * t8085;
    (t103452, t103586, t104115, t106589, t107923, t109269, t109731)
}
