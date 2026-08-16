//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 693/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk693(t3193: f64, t3206: f64, t151: f64, t816: f64, t3199: f64, t22: f64, t955: f64, t963: f64, t15: f64, t26: f64, t146: f64, t213: f64, t3: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12454 = t3193 * t3206;
    let t12459 = t816 * t151;
    let t12460 = t3199 * t12459;
    let t12462 = t22 * t963 * t955;
    let t12467 = 1.0_f64 / t15 / t26 / 4.0_f64;
    let t12468 = t12467 * t146;
    let t12469 = t3 * t213;
    (t12454, t12459, t12460, t12462, t12467, t12468, t12469)
}
