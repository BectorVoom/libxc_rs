//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1495/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1495(t10069: f64, t22361: f64, t22365: f64, t14239: f64, t14242: f64, t10023: f64, t22314: f64, t2470: f64, t3999: f64, t6888: f64, t4086: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75145 = t10069 * t22361;
    let t75147 = t10069 * t22365;
    let t75176 = t14239 * t14242;
    let t75179 = t10023 * t22314 * t2470;
    let t75228 = t3999 * t6888;
    let t75251 = t786 * t4086 * t6888;
    (t75145, t75147, t75176, t75179, t75228, t75251)
}
