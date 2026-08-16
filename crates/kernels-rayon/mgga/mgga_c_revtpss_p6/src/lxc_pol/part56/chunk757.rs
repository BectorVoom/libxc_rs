//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 757/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk757(t2033: f64, t4147: f64, t2121: f64, t8435: f64, t2247: f64, t1937: f64, t7586: f64, t1936: f64, t2163: f64) -> (f64, f64, f64, f64, f64) {
    let t8717 = t4147 * t2033;
    let t8736 = t8435 * t2121;
    let t8737 = t2247 * t8736;
    let t8743 = t7586 * t1937;
    let t8749 = t2163 * t1936;
    (t8717, t8736, t8737, t8743, t8749)
}
