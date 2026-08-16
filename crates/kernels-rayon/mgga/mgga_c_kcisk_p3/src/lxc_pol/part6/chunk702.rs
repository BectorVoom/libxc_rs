//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 702/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk702(t2921: f64, t846: f64, t2912: f64, t2918: f64, t157: f64, t2903: f64, t2856: f64, t2879: f64, t831: f64, t32: f64, t5: f64, t969: f64) -> (f64, f64, f64, f64) {
    let t12572 = t2921 * t846;
    let t12573 = t2918 * t2912 * t12572;
    let t12576 = t157 * t2903;
    let t12581 = 6.0_f64 * t2856 * t831 * t2879;
    let t12584 = 0.34451131037037037036e-2_f64 * t5 * t969 * t32;
    (t12573, t12576, t12581, t12584)
}
