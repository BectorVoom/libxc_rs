//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 408/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk408(t12: f64, t20: f64, t2863: f64, t2866: f64, t824: f64, t22: f64, t964: f64) -> (f64, f64, f64, f64) {
    let t2871 = 1.0_f64/f64::sqrt(t12);
    let t2872 = t2871 * t20;
    let t2873 = t2872 * t2863;
    let t2875 = t824 * t2866;
    let t2877 = t22 * t964;
    (t2872, t2873, t2875, t2877)
}
