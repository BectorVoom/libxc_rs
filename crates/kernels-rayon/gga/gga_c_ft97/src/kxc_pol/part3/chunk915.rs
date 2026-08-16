//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 915/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk915(t18127: f64, t200: f64, t236: f64, t4977: f64, t3724: f64, t13443: f64, t17993: f64, t17994: f64, t17997: f64, t18003: f64, t18007: f64, t18012: f64, t18015: f64, t18018: f64, t18021: f64, t18024: f64, t18084: f64, t18090: f64, t224: f64, t2387: f64, t3723: f64, t3789: f64, t4986: f64, t678: f64, t680: f64, t690: f64, t695: f64, t709: f64, t710: f64) -> f64 {
    let t18128 = t18127 * t200;
    let t18132 = t236 * t4977;
    let t18133 = t3724 * t18132;
    let t18136 = -0.2370952259137005195e-1_f64 * t17993 * t17994 - 6.0_f64 * t3789 * t17997 * t709 + 0.2370952259137005195e-1_f64 * t13443 * t18003 + 0.11627450473218896e-1_f64 * t2387 * t18007 - 0.32253953169881963531e-5_f64 * t678 * t18012 + 0.23254900946437792e-2_f64 * t678 * t18015 - 0.279058811357253504e-2_f64 * t678 * t18018 + 0.46509801892875584e-2_f64 * t678 * t18021 - 0.11619434043764639964e-3_f64 * t678 * t18024 - t224 * t695 * t18084 - 2.0_f64 * t4986 * t710 - 0.23254900946437792e-1_f64 * t18090 * t690 - 0.11627450473218896e-1_f64 * t678 * t680 * t18128 + 0.67598802253579164263e-4_f64 * t3723 * t18133;
    t18136
}
