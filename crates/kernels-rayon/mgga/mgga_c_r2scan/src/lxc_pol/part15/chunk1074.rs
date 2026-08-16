//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1074/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1074(t120: f64, t2090: f64, t3294: f64, t3296: f64, t2096: f64, t2167: f64, t546: f64, t10729: f64, t565: f64, t10711: f64, t10734: f64, t547: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37932 = t120 * t2090 * t3294;
    let t37933 = t37932 * t3296;
    let t37935 = t2167 * t2096;
    let t37936 = t546 * t37935;
    let t37937 = t37936 * t10729;
    let t37939 = t565 * t37935;
    let t37940 = t37939 * t10711;
    let t37942 = t547 * t10734;
    (t37932, t37933, t37936, t37937, t37939, t37940, t37942)
}
