//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 622/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk622(t43: f64, t40: f64, t4064: f64, t483: f64, t803: f64, t2898: f64, t474: f64, t34: f64, t817: f64, t1281: f64, t1284: f64, t292: f64, t39: f64, t4000: f64, t818: f64, t821: f64, t824: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t4065 = t40 * t4064;
    let t4068 = t483 * t803;
    let t4069 = t40 * t4068;
    let t4070 = t2898 * t474;
    let t4073 = t817 * t34;
    let t4083 = piecewise3(t44, 0.0_f64, 8.0_f64 / 27.0_f64 * t4070 * t818 - 8.0_f64 / 9.0_f64 * t4073 * t4000 - 2.0_f64 / 9.0_f64 * t1281 * t824 + 4.0_f64 / 3.0_f64 * t292 * t821 - 4.0_f64 * t1284 * t39);
    (t4065, t4068, t4069, t4070, t4083)
}
