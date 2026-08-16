//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 981/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk981(t33591: f64, t651: f64, t5542: f64, t8595: f64, t2014: f64, t1868: f64, t4147: f64, t32119: f64, t1937: f64, t28030: f64, t1518: f64, t1931: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33592 = t651 * t33591;
    let t33594 = t8595 * t5542;
    let t33595 = t2014 * t33594;
    let t33596 = t4147 * t1868;
    let t33597 = t32119 * t33596;
    let t33599 = 3.0_f64 * t2014 * t33597;
    let t33600 = t28030 * t1937;
    let t33602 = t1931 * t1518;
    (t33592, t33594, t33595, t33597, t33599, t33600, t33602)
}
