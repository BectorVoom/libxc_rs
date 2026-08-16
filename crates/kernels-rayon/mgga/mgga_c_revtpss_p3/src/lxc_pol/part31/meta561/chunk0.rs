//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1972/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1972(t10308: f64, t1466: f64, t21661: f64, t602: f64, t2246: f64, t5812: f64, t10871: f64, t5977: f64, t18493: f64, t221: f64, t18498: f64, t6016: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t60224 = t1466 * t10308;
    let t60670 = t21661 * t602;
    let t60673 = t5812 * t2246;
    let t61532 = t5977 * t10871;
    let t61639 = t221 * t18493;
    let t61725 = t221 * t18498;
    let t61749 = t6016 * t836;
    (t60224, t60670, t60673, t61532, t61639, t61725, t61749)
}
