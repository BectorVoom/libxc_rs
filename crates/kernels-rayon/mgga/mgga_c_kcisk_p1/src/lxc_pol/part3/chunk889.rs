//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 889/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk889(t13312: f64, t3482: f64, t12819: f64, t12822: f64, t12834: f64, t12836: f64, t12838: f64, t12842: f64, t13291: f64, t13297: f64, t13302: f64, t13307: f64, t13309: f64) -> (f64, f64) {
    let t13313 = t3482 * t13312;
    let t13315 = 0.49745833333333333332e-2_f64 * t12819 + 0.49745833333333333332e-2_f64 * t12822 + 0.73697530864197530861e-2_f64 * t12834 + 0.66327777777777777776e-2_f64 * t12836 + 0.33163888888888888887e-2_f64 * t12838 + 0.55273148148148148145e-2_f64 * t12842 - 0.24872916666666666666e-2_f64 * t13291 - 0.66327777777777777775e-2_f64 * t13297 + 0.99491666666666666664e-2_f64 * t13302 - 0.99491666666666666664e-2_f64 * t13307 + 0.66327777777777777776e-2_f64 * t13309 - 0.17687407407407407407e-1_f64 * t13313;
    (t13313, t13315)
}
