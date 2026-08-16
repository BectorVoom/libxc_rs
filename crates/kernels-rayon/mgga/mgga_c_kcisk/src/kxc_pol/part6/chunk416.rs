//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 416/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk416(t2947: f64, t73: f64, t879: f64, t880: f64, t20: f64, t71: f64, t74: f64, t79: f64, t2863: f64, t2866: f64, t866: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2948 = 1.0_f64 / t2947;
    let t2949 = t73 * t2948;
    let t2950 = t879 * t879;
    let t2951 = t2950 * t880;
    let t2957 = 1.0_f64 / t74 / t71 * t79 * t20;
    let t2958 = t2957 * t2863;
    let t2960 = t866 * t2866;
    let t2962 = t68 * t2866;
    (t2948, t2949, t2950, t2951, t2957, t2958, t2960, t2962)
}
