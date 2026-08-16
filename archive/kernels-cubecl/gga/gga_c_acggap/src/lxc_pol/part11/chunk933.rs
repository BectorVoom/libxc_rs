//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 933/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk933<F: Float>(t7310: F, t7487: F, t2082: F, t30044: F, t2087: F, t7610: F, t1092: F, t7605: F, t381: F, t7779: F, t2100: F, t1096: F, t1983: F, t7380: F) -> (F, F, F, F, F, F, F) {
    let t31279 = t7310 * t7487;
    let t31283 = t30044 * t2082;
    let t31284 = F::cast_from(0.32155513588552302729e-3_f64) * t31283;
    let t31285 = t7610 * t2087;
    let t31287 = t7605 * t1092;
    let t31289 = t381 * t7779;
    let t31290 = t31289 * t2100;
    let t31291 = F::cast_from(0.19812298142450615803e-1_f64) * t31290;
    let t31293 = t7380 * t1983 * t1096;
    (t31279, t31284, t31285, t31287, t31289, t31291, t31293)
}
