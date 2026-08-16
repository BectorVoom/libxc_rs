//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 672/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk672(t140: f64, t3737: f64, t4594: f64, t139: f64, t172: f64, t79: f64, t721: f64, t167: f64, t3281: f64, t1394: f64, t298: f64, t569: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10494 = t140 * t3737 * t4594;
    let t10500 = t139 * t172 * t79;
    let t10501 = t10500 * t721;
    let t10502 = 0.73697530864197530862e-3_f64 * t10501;
    let t10519 = 6.0_f64 * t167;
    let t10520 = 6.0_f64 * t3281;
    let t10568 = t298 * t1394 * t569;
    (t10494, t10500, t10501, t10502, t10519, t10520, t10568)
}
