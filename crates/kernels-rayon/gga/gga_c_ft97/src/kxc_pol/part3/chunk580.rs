//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 580/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk580(t167: f64, t4462: f64, t569: f64, t2205: f64, t4454: f64, t1039: f64, t2086: f64, t91: f64, t2097: f64, t4511: f64, t2102: f64, t4656: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4743 = t569 * t167 * t4462;
    let t4747 = t2205 * t167 * t4454;
    let t4753 = t1039 * t1039;
    let t4755 = t91 * t2086 * t4753;
    let t4759 = t2097 * t4511;
    let t4762 = t2102 * t4656;
    (t4743, t4747, t4753, t4755, t4759, t4762)
}
