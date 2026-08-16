//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 1010/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk1010(t17753: f64, t4199: f64, t19263: f64, t2771: f64, t1775: f64, t5346: f64, t458: f64, t5360: f64, t5356: f64, t19011: f64, t10603: f64, t19016: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19643 = t4199 * t17753;
    let t19646 = t2771 * t19263;
    let t19649 = t1775 * t5346;
    let t19651 = t458 * t5360;
    let t19653 = t458 * t5356;
    let t19656 = t2771 * t19011;
    let t19659 = t10603 * t19016;
    (t19643, t19646, t19649, t19651, t19653, t19656, t19659)
}
