//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 927/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk927(t33877: f64, t34075: f64, t3: f64, t2039: f64, t7801: f64, t1458: f64, t16524: f64, t24465: f64, t27254: f64, t32295: f64, t33185: f64, t33192: f64, t33195: f64, t3941: f64, t577: f64, t7230: f64, t7956: f64, t8508: f64, t8717: f64) -> (f64, f64, f64, f64) {
    let t34076 = t33877 + t34075;
    let t34077 = t3 * t34076;
    let t34099 = t2039 * t7801;
    let t34102 = 0.45e1_f64 * t34076 * t577 + 0.135e2_f64 * t32295 * t1458 + 27.0_f64 * t27254 * t2039 + 54.0_f64 * t24465 * t7956 + 27.0_f64 * t7230 * t7801 + 27.0_f64 * t16524 * t8717 + 27.0_f64 * t33185 * t8717 + 54.0_f64 * t3941 * t34099 + t33192 + t33195 + t8508;
    (t34076, t34077, t34099, t34102)
}
