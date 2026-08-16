//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1061/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1061(t1236: f64, t371: f64, t676: f64, t1235: f64, t12627: f64, t225: f64, t480: f64, t12629: f64, t482: f64, t372: f64, t127: f64, t3672: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12984 = t371 * t676 * t1236;
    let t12985 = t1235 * t12984;
    let t12987 = t12627 * t225;
    let t12988 = t12987 * t480;
    let t12989 = t482 * t12629;
    let t12991 = t371 * t372 * t12989;
    let t12995 = t371 * t127 * t3672;
    (t12984, t12985, t12987, t12988, t12991, t12995)
}
