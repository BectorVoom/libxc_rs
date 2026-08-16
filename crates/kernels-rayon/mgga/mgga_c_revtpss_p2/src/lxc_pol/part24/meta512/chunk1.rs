//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1529/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1529(t15707: f64, t19920: f64, t23891: f64, t3127: f64, t3172: f64, t19697: f64, t4820: f64, t1032: f64, t1040: f64, t23959: f64, t19658: f64, t4879: f64) -> (f64, f64, f64, f64, f64) {
    let t78910 = t15707 * t19920;
    let t78915 = t3127 * t3172 * t23891;
    let t78986 = t19697 * t4820;
    let t79038 = t23959 * t1032 * t1040;
    let t79071 = t4879 * t19658;
    (t78910, t78915, t78986, t79038, t79071)
}
