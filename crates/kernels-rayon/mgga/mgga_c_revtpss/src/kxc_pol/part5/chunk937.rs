//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 937/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk937(t3: f64, t6936: f64, t116: f64, t5883: f64, t117: f64, t5920: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t159: f64, t793: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6937 = t3 * t6936;
    let t6941 = param_d * t6936;
    let t6945 = t116 * t5883;
    let t6948 = t117 * t5920;
    let t6951 = 6.0_f64 * t1916 * t1918 + 6.0_f64 * t572 * t6945 + 3.0_f64 * t572 * t6948 + t573 * t6941;
    let t7021 = t793 * t159;
    (t6937, t6941, t6945, t6948, t6951, t7021)
}
