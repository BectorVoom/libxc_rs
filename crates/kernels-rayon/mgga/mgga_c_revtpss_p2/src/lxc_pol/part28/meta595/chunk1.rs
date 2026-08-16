//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2068/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2068(t1426: f64, t7275: f64, t786: f64, t3917: f64, t25953: f64, t26072: f64, t2435: f64, t25913: f64, t7289: f64, t94600: f64, t2028: f64, t3999: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t94748 = t786 * t7275 * t1426;
    let t94749 = t94748 * t3917;
    let t94756 = t26072 * t25953;
    let t94758 = t2435 * t25913;
    let t94761 = 0.39982213492741449076e-1_f64 * t7289 * t94600;
    let t94762 = t2028 * t3999;
    (t94748, t94749, t94756, t94758, t94761, t94762)
}
