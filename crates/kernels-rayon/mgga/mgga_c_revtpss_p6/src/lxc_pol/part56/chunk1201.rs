//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1201/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1201(t125182: f64, t125184: f64, t132135: f64, t132141: f64, t132144: f64, t132152: f64, t132167: f64, t1456: f64, t1458: f64, t1914: f64, t2168: f64, t29490: f64, t33572: f64, t35034: f64, t5790: f64, t7691: f64, t7700: f64, t8241: f64, t8249: f64, t8978: f64) -> f64 {
    let t132170 = 2.0_f64 * t8241 * t7700 + 2.0_f64 * t132135 + 2.0_f64 * t7691 * t8249 + 2.0_f64 * t2168 * t29490 + t125182 + t125184 + t132141 + t5790 * t8978 + t1914 * t33572 + t132144 + t1456 * t35034 + t1458 * (t132152 + t132167);
    t132170
}
