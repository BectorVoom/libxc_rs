//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1574/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1574(t22789: f64, t72: f64, t757: f64, t1317: f64, t22790: f64, t1320: f64, t512: f64, t749: f64, t221: f64, t22954: f64, t4018: f64, t4019: f64) -> (f64, f64, f64, f64, f64) {
    let t85912 = t22789 * t72 * t757;
    let t85929 = t1317 * t22790;
    let t85931 = t1320 * t22790;
    let t85986 = t512 * t22789 * t749;
    let t86061 = t4018 * t4019 * t221 * t22954;
    (t85912, t85929, t85931, t85986, t86061)
}
