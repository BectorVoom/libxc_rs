//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2791/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2791(t46445: f64, t2517: f64, t2658: f64, t5392: f64, t47160: f64, t41291: f64, t16634: f64, t2427: f64, t47163: f64, t47165: f64, t12923: f64, t3966: f64, t4194: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t59011 = 24.0_f64 * t46445;
    let t59013 = t2658 * t2517 * t5392;
    let t59014 = 12.0_f64 * t59013;
    let t59015 = 2.0_f64 * t47160;
    let t59016 = 8.0_f64 * t41291;
    let t59018 = 8.0_f64 * t2427 * t16634;
    let t59019 = 16.0_f64 * t47163;
    let t59020 = 16.0_f64 * t47165;
    let t59022 = t4194 * t12923 * t3966;
    (t59011, t59014, t59015, t59016, t59018, t59019, t59020, t59022)
}
