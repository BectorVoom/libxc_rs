//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1488/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1488(t11643: f64, t11994: f64, t12025: f64, t3127: f64, t3172: f64, t3105: f64, t3196: f64, t11656: f64, t2852: f64, t3154: f64, t2251: f64, t11648: f64, t3124: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42190 = t11994 * t11643;
    let t42193 = t3127 * t3172 * t12025;
    let t42195 = t3196 * t3105;
    let t42204 = t11656 * t11643;
    let t42215 = t3154 * t2852;
    let t42216 = t42215 * t2251;
    let t42227 = t3124 * t11648;
    (t42190, t42193, t42195, t42204, t42216, t42227)
}
