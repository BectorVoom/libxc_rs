//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2117/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2117(t2770: f64, t340: f64, t2403: f64, t4389: f64, t4386: f64, t344: f64, t42308: f64, t60: f64, t10213: f64, t134: f64, t4509: f64, t4540: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t48143 = t340 * t2770;
    let t48155 = t2403 * t4389;
    let t48156 = 10.0_f64 / 9.0_f64 * t48155;
    let t48157 = t2403 * t4386;
    let t48158 = 5.0_f64 / 27.0_f64 * t48157;
    let t48180 = t60 * t42308 * t344;
    let t48213 = t134 * t10213 * t344;
    let t48217 = t4509 * t4540;
    (t48143, t48155, t48156, t48157, t48158, t48180, t48213, t48217)
}
