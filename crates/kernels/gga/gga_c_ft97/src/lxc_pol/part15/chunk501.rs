//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 501/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk501<F: Float>(t5049: F, t695: F, t1111: F, t1115: F, t1417: F, t1701: F, t224: F, t238: F, t2384: F, t2387: F, t3759: F, t3766: F, t4940: F, t4943: F, t4949: F, t4953: F, t4957: F, t4961: F, t4979: F, t4982: F, t4987: F, t4991: F, t5003: F, t5007: F, t5016: F, t5019: F, t5026: F, t678: F, t680: F) -> (F, F) {
    let t5050 = t695 * t5049;
    let t5052 = F::new(0.67598802253579164263e-4) * t4940 * t2384 - F::new(0.46509801892875584e-1) * t3759 * t680 * t4943 - F::new(0.13784064983740990796e-3) * t4949 * t4953 + F::new(0.23254900946437792e-1) * t2387 * t4957 + F::new(0.23254900946437792e-2) * t678 * t4961 - F::new(0.11627450473218896e-1) * t678 * t4979 + F::new(0.19365723406274399941e-3) * t678 * t4982 + F::new(2.0) * t4987 + F::new(0.2370952259137005195e-1) * t1115 * t1111 - F::new(4.0) * t3766 * t4991 + F::new(2.0) * t5007 + F::new(0.14053536537767171586e-3) * t238 * t5016 - F::new(0.11854761295685025975e-1) * t1417 * t1701 * t5019 - F::new(0.37540077436335915588e-1) * t238 * t5003 + F::new(2.0) * t224 * t5026 - t224 * t5050;
    (t5050, t5052)
}
