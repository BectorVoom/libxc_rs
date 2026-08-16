//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1228/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1228(t13176: f64, t816: f64, t1512: f64, t9671: f64, t2697: f64, t4257: f64, t2563: f64, t4159: f64, t4155: f64, t9573: f64, t2644: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13177 = t13176 * t816;
    let t13182 = t9671 * t1512;
    let t13190 = 35.0_f64 / 576.0_f64 * t2697 * t4257;
    let t13202 = 7.0_f64 / 72.0_f64 * t2563 * t4159;
    let t13208 = 7.0_f64 / 24.0_f64 * t9573 * t4155;
    let t13222 = t2644 * t820;
    (t13177, t13182, t13190, t13202, t13208, t13222)
}
