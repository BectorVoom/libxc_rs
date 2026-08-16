//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1081/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1081(t10868: f64, t2147: f64, t6541: f64, t6402: f64, t10844: f64, t10903: f64, t2201: f64, t10848: f64, t2207: f64, t10698: f64, t10716: f64, t10810: f64, t1577: f64, t6536: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38088 = t2147 * t10868 * t6541;
    let t38093 = t2147 * t10868 * t6402;
    let t38096 = t2201 * t10903 * t10844;
    let t38099 = t2207 * t10903 * t10848;
    let t38111 = t10698 * t10716;
    let t38114 = t1577 * t10810 * t6536;
    (t38088, t38093, t38096, t38099, t38111, t38114)
}
