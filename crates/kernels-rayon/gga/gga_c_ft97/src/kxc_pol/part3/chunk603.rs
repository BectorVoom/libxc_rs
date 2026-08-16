//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 603/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk603(t5049: f64, t695: f64, t1111: f64, t1115: f64, t1417: f64, t1701: f64, t224: f64, t238: f64, t2384: f64, t2387: f64, t3759: f64, t3766: f64, t4940: f64, t4943: f64, t4949: f64, t4953: f64, t4957: f64, t4961: f64, t4979: f64, t4982: f64, t4987: f64, t4991: f64, t5003: f64, t5007: f64, t5016: f64, t5019: f64, t5026: f64, t678: f64, t680: f64) -> f64 {
    let t5050 = t695 * t5049;
    let t5052 = 0.67598802253579164263e-4_f64 * t4940 * t2384 - 0.46509801892875584e-1_f64 * t3759 * t680 * t4943 - 0.13784064983740990796e-3_f64 * t4949 * t4953 + 0.23254900946437792e-1_f64 * t2387 * t4957 + 0.23254900946437792e-2_f64 * t678 * t4961 - 0.11627450473218896e-1_f64 * t678 * t4979 + 0.19365723406274399941e-3_f64 * t678 * t4982 + 2.0_f64 * t4987 + 0.2370952259137005195e-1_f64 * t1115 * t1111 - 4.0_f64 * t3766 * t4991 + 2.0_f64 * t5007 + 0.14053536537767171586e-3_f64 * t238 * t5016 - 0.11854761295685025975e-1_f64 * t1417 * t1701 * t5019 - 0.37540077436335915588e-1_f64 * t238 * t5003 + 2.0_f64 * t224 * t5026 - t224 * t5050;
    t5052
}
