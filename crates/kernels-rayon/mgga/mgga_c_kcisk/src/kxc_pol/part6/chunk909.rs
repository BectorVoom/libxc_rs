//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 909/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk909(t7099: f64, t8708: f64, t23460: f64, t23606: f64, t23609: f64, t29082: f64, t29085: f64, t29091: f64, t29097: f64, t29152: f64, t29155: f64, t29161: f64, t29164: f64, t29166: f64, t29168: f64) -> (f64, f64) {
    let t29170 = t7099 * t8708;
    let t29172 = -0.33547222222222222222e0_f64 * t29082 + 0.12077e1_f64 * t29085 - 0.181155e1_f64 * t29091 - 0.301925e0_f64 * t29097 - 0.73586666666666666666e-1_f64 * t29152 - 0.16557e0_f64 * t29155 + 0.20128333333333333333e0_f64 * t23460 + 0.11038e0_f64 * t23606 + 0.33114e0_f64 * t23609 + 0.33114e0_f64 * t29161 - 0.99342e0_f64 * t29164 + 0.16504875e0_f64 * t29166 + 0.247573125e0_f64 * t29168 - 0.3883875e1_f64 * t29170;
    (t29170, t29172)
}
