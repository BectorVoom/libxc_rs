//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2437/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2437(t10402: f64, t11037: f64, t2402: f64, t973: f64, t999: f64, t1030: f64, t10477: f64, t10472: f64, t10475: f64, t3128: f64, t10903: f64, t10948: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42546 = t11037 * t10402;
    let t42552 = t973 * t2402 * t999;
    let t42559 = t1030 * t10477;
    let t42561 = t10472 * t10475 * t42559;
    let t42565 = t10472 * t3128 * t42559;
    let t42570 = t10948 * t10903;
    (t42546, t42552, t42559, t42561, t42565, t42570)
}
