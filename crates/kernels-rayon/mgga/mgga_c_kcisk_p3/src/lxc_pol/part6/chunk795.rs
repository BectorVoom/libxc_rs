//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 795/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk795(t1646: f64, t8533: f64, t3521: f64, t8896: f64, t8908: f64, t8912: f64, t8920: f64, t827: f64, t8564: f64, t8567: f64, t8570: f64, t45: f64, t8584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22599 = t1646 * t8533;
    let t22646 = t3521 * t8896;
    let t22652 = t3521 * t8908;
    let t22654 = t3521 * t8912;
    let t22656 = t3521 * t8920;
    let t22698 = t827 * t8564;
    let t22705 = t827 * t8567;
    let t22707 = t827 * t8570;
    let t22750 = t45 * t8584;
    (t22599, t22646, t22652, t22654, t22656, t22698, t22705, t22707, t22750)
}
