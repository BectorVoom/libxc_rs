//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1006/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1006(t1937: f64, t34446: f64, t7586: f64, t7735: f64, t1936: f64, t29427: f64, t7741: f64, t7901: f64, t8764: f64, t2042: f64, t8245: f64, t2170: f64, t7950: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34447 = t34446 * t1937;
    let t34449 = t7586 * t7735;
    let t34453 = t29427 * t1936;
    let t34455 = t34446 * t1936;
    let t34457 = t7586 * t7741;
    let t34464 = t8764 * t7901;
    let t34481 = t8245 * t2042;
    let t34483 = t2170 * t7950;
    (t34447, t34449, t34453, t34455, t34457, t34464, t34481, t34483)
}
