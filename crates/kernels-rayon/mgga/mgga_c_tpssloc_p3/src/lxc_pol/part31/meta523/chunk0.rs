//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1737/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1737(t29210: f64, t29394: f64, t3: f64, t1458: f64, t7801: f64, t2039: f64, t5493: f64, t1401: f64, t16524: f64, t20162: f64, t24465: f64, t27254: f64, t28893: f64, t28951: f64, t3941: f64, t5371: f64, t5456: f64, t577: f64, t7230: f64, t7956: f64) -> (f64, f64, f64, f64, f64) {
    let t29395 = t29210 + t29394;
    let t29396 = t3 * t29395;
    let t29422 = t7801 * t1458;
    let t29425 = t2039 * t5493;
    let t29430 = 0.45e1_f64 * t29395 * t577 + 27.0_f64 * t27254 * t1458 + 27.0_f64 * t24465 * t5456 + 0.135e2_f64 * t7230 * t5493 + 0.135e2_f64 * t20162 * t2039 + 54.0_f64 * t16524 * t7956 + 27.0_f64 * t5371 * t7801 + 27.0_f64 * t28893 * t2039 + 54.0_f64 * t3941 * t29422 + 27.0_f64 * t3941 * t29425 + 0.135e2_f64 * t1401 * t28951;
    (t29395, t29396, t29422, t29425, t29430)
}
