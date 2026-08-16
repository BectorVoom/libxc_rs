//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 982/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk982(t22116: f64, t8959: f64, t22100: f64, t39942: f64, t21130: f64, t703: f64, t801: f64, t1109: f64, t5295: f64, t21249: f64, t816: f64, t5260: f64, t817: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83088 = t8959 * t22116;
    let t83103 = 0.22136921132726965153e-3_f64 * t39942 * t22100;
    let t83109 = t703 * t21130 * t801;
    let t83158 = t1109 * t5295;
    let t83210 = t816 * t21249;
    let t83232 = t817 * t5260;
    (t83088, t83103, t83109, t83158, t83210, t83232)
}
