//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 691/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk691(t181: f64, t3086: f64, t3088: f64, t955: f64, t21: f64, t3117: f64, t3201: f64, t142: f64, t3107: f64, t5: f64, t1016: f64, t4: f64, t918: f64) -> (f64, f64, f64, f64, f64) {
    let t12434 = t181 * t3086;
    let t12435 = t3088 * t955;
    let t12436 = t12434 * t12435;
    let t12442 = t3201 * t21 * t3117;
    let t12446 = t5 * t142 * t3107;
    let t12450 = t1016 * t4 * t918;
    (t12435, t12436, t12442, t12446, t12450)
}
