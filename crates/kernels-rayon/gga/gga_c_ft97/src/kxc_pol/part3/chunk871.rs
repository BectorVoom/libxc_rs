//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 871/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk871(t17509: f64, t17522: f64, t184: f64, t21: f64, t3658: f64, t1079: f64, t4431: f64, t649: f64, t920: f64, t3663: f64, t1578: f64, t4888: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17523 = t17509 + t17522;
    let t17524 = t17523 * t184;
    let t17531 = t21 * t3658;
    let t17532 = t1079 * t17531;
    let t17535 = t649 * t4431;
    let t17538 = t184 * t920;
    let t17539 = t3663 * t17538;
    let t17542 = t1079 * t1578;
    let t17544 = t4888 * t648;
    (t17524, t17532, t17535, t17539, t17542, t17544)
}
