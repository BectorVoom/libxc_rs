//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1043/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1043(t32118: f64, t32123: f64, t32124: f64, t32126: f64, t32131: f64, t32182: f64, t32856: f64, t32858: f64, t32862: f64, t32864: f64, t32867: f64, t32869: f64, t651: f64, t7007: f64, t7586: f64) -> f64 {
    let t32873 = -2.0_f64 * t32869 * t651 - 2.0_f64 * t7007 * t7586 - t32118 - t32123 - t32124 + 3.0_f64 * t32126 + t32131 + t32182 - 2.0_f64 * t32856 - 2.0_f64 * t32858 - 2.0_f64 * t32862 - 2.0_f64 * t32864 - 2.0_f64 * t32867;
    t32873
}
