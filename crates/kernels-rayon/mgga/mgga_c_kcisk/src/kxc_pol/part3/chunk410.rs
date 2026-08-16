//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 410/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk410(t3063: f64, t933: f64, t177: f64, t918: f64, t140: f64, t191: f64, t119: f64, t974: f64, t139: f64, t172: f64, t1003: f64, t167: f64, t944: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3064 = t3063 * t933;
    let t3069 = t918 * t177;
    let t3071 = t140 * t3069 * t191;
    let t3073 = t119 * t974;
    let t3075 = t140 * t3073 * t191;
    let t3077 = t139 * t172;
    let t3078 = t3077 * t1003;
    let t3082 = t167 * t944;
    (t3064, t3071, t3075, t3077, t3078, t3082)
}
