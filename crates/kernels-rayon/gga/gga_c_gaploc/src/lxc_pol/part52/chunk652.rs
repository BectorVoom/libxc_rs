//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 652/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk652(t11981: f64, t447: f64, t1445: f64, t11987: f64, t1457: f64, t10617: f64, t10620: f64, t1450: f64, t1456: f64, t9549: f64, t9553: f64, t9556: f64, t9560: f64, t9564: f64, t9568: f64, t9571: f64, t9575: f64, t9577: f64, t9579: f64, t9582: f64, t9584: f64) -> f64 {
    let t12134 = t11981 * t447;
    let t12135 = t1445 * t12134;
    let t12138 = t1457 * t11987;
    let t12145 = -t10617 + t10620 - 0.23005755572352449806e1_f64 * t1450 * t12135 + 0.35750489951850426669e0_f64 * t1456 * t12138 + 0.85206502119823888171e-1_f64 * t9549 - 0.51123901271894332903e0_f64 * t9553 + 0.51123901271894332903e0_f64 * t9556 + t9560 - t9564 - 0.38342925953920749677e0_f64 * t9568 - t9571 - t9575 + t9577 + t9579 + t9582 - t9584;
    t12145
}
