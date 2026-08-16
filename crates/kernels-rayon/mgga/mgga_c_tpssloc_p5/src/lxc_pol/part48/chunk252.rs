//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 252/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk252(t1190: f64, t491: f64, t1169: f64, t221: f64, t456: f64, t1089: f64, t1176: f64, t607: f64, t974: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1191 = t1190 * t491;
    let t1193 = t221 * t1169;
    let t1195 = t456 * t1193 / 288.0_f64;
    let t1196 = t1176 * t1089;
    let t1197 = t1196 * t607;
    let t1198 = t974 * t1197;
    let t1201 = t1190 * t225;
    (t1191, t1193, t1195, t1196, t1197, t1198, t1201)
}
