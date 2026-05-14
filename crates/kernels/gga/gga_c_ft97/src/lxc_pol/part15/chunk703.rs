//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 703/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk703<F: Float>(t1113: F, t4978: F, t1096: F, t5005: F, t680: F, t4960: F, t21271: F, t2379: F, t1127: F, t17965: F, t4977: F, t2394: F, t5019: F, t7853: F, t200: F, t21130: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21277 = t4978 * t1113;
    let t21281 = t1096 * t5005;
    let t21282 = t680 * t21281;
    let t21285 = t4960 * t1113;
    let t21289 = t2379 * t21271;
    let t21292 = t17965 * t1127;
    let t21296 = t1096 * t4977;
    let t21297 = t2394 * t21296;
    let t21300 = t2379 * t21296;
    let t21306 = t7853 * t5019;
    let t21309 = t21130 * t200;
    (t21277, t21281, t21282, t21285, t21289, t21292, t21296, t21297, t21300, t21306, t21309)
}
