//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 603/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk603<F: Float>(t5049: F, t695: F, t1111: F, t1115: F, t1417: F, t1701: F, t224: F, t238: F, t2384: F, t2387: F, t3759: F, t3766: F, t4940: F, t4943: F, t4949: F, t4953: F, t4957: F, t4961: F, t4979: F, t4982: F, t4987: F, t4991: F, t5003: F, t5007: F, t5016: F, t5019: F, t5026: F, t678: F, t680: F) -> F {
    let t5050 = t695 * t5049;
    let t5052 = F::cast_from(0.67598802253579164263e-4_f64) * t4940 * t2384 - F::cast_from(0.46509801892875584e-1_f64) * t3759 * t680 * t4943 - F::cast_from(0.13784064983740990796e-3_f64) * t4949 * t4953 + F::cast_from(0.23254900946437792e-1_f64) * t2387 * t4957 + F::cast_from(0.23254900946437792e-2_f64) * t678 * t4961 - F::cast_from(0.11627450473218896e-1_f64) * t678 * t4979 + F::cast_from(0.19365723406274399941e-3_f64) * t678 * t4982 + F::cast_from(2.0_f64) * t4987 + F::cast_from(0.2370952259137005195e-1_f64) * t1115 * t1111 - F::cast_from(4.0_f64) * t3766 * t4991 + F::cast_from(2.0_f64) * t5007 + F::cast_from(0.14053536537767171586e-3_f64) * t238 * t5016 - F::cast_from(0.11854761295685025975e-1_f64) * t1417 * t1701 * t5019 - F::cast_from(0.37540077436335915588e-1_f64) * t238 * t5003 + F::cast_from(2.0_f64) * t224 * t5026 - t224 * t5050;
    t5052
}
