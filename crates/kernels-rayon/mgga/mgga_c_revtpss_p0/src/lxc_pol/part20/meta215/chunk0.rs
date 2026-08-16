//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1000/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1000(t10518: f64, t2798: f64, t2722: f64, t675: f64, t231: f64, t268: f64, t251: f64, t4503: f64, t786: f64, t2723: f64, t2453: f64, t2797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10519 = t2798 * t10518;
    let t10521 = t675 * t2722;
    let t10523 = t268 * t10521 * t231;
    let t10524 = t2798 * t10523;
    let t10529 = t4503 * t251;
    let t10530 = t786 * t10529;
    let t10532 = t268 * t10521 * t2723;
    let t10533 = t10530 * t10532;
    let t10535 = t2453 * t2797;
    (t10519, t10523, t10524, t10529, t10530, t10532, t10533, t10535)
}
