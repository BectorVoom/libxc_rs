//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 768/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk768(t1113: f64, t4978: f64, t1096: f64, t5005: f64, t680: f64, t4960: f64, t21271: f64, t2379: f64, t1127: f64, t17965: f64, t4977: f64, t2394: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21277 = t4978 * t1113;
    let t21281 = t1096 * t5005;
    let t21282 = t680 * t21281;
    let t21285 = t4960 * t1113;
    let t21289 = t2379 * t21271;
    let t21292 = t17965 * t1127;
    let t21296 = t1096 * t4977;
    let t21297 = t2394 * t21296;
    (t21277, t21281, t21282, t21285, t21289, t21292, t21296, t21297)
}
