//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2893/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2893(t14379: f64, t49226: f64, t2791: f64, t5689: f64, t2794: f64, t4433: f64, t2792: f64, t2836: f64, t5727: f64, t10661: f64, t17520: f64, t2793: f64) -> (f64, f64, f64, f64, f64) {
    let t60354 = 24.0_f64 * t49226 * t14379;
    let t60357 = t5689 * t2791;
    let t60359 = 2.0_f64 * t60357 * t2794;
    let t60360 = t4433 * t4433;
    let t60371 = 2.0_f64 * t2792 * t5727 * t2836;
    let t60374 = 0.96491876992155210402e2_f64 * t10661 * t17520 * t2793;
    (t60354, t60359, t60360, t60371, t60374)
}
