//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 697/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk697(t1043: f64, t2775: f64, t3961: f64, t4582: f64, t2770: f64, t3061: f64, t1615: f64, t376: f64, t1022: f64, t3131: f64, t1023: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4583 = t1043 * t2775;
    let t4584 = t4583 * t3961;
    let t4585 = t4582 * t4584;
    let t4588 = t3061 * t2770;
    let t4589 = t4588 * t3961;
    let t4590 = t4582 * t4589;
    let t4593 = t376 * t1615;
    let t4594 = t3131 * t1022;
    let t4595 = t4593 * t4594;
    let t4596 = t4582 * t4595;
    let t4599 = t4593 * t1023;
    let t4600 = t4582 * t4599;
    (t4583, t4584, t4585, t4588, t4589, t4590, t4593, t4594, t4595, t4596, t4599, t4600)
}
