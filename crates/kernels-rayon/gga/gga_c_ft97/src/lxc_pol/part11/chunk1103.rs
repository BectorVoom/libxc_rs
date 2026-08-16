//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1103/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1103(t10866: f64, t8675: f64, t10871: f64, t10847: f64, t10877: f64, t10862: f64, t2925: f64, t39448: f64, t10859: f64, t2938: f64, t703: f64, t10855: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43146 = t8675 * t10866;
    let t43148 = t8675 * t10871;
    let t43150 = t8675 * t10847;
    let t43152 = t8675 * t10877;
    let t43158 = t8675 * t10862;
    let t43160 = t39448 * t2925;
    let t43162 = t8675 * t10859;
    let t43164 = t703 * t2938;
    let t43177 = t8675 * t10855;
    (t43146, t43148, t43150, t43152, t43158, t43160, t43162, t43164, t43177)
}
