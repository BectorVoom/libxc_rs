//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1022/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1022(t1043: f64, t2775: f64, t3961: f64, t4582: f64, t2770: f64, t3061: f64, t1615: f64, t376: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4583 = t1043 * t2775;
    let t4584 = t4583 * t3961;
    let t4585 = t4582 * t4584;
    let t4588 = t3061 * t2770;
    let t4589 = t4588 * t3961;
    let t4590 = t4582 * t4589;
    let t4593 = t376 * t1615;
    (t4583, t4584, t4585, t4588, t4589, t4590, t4593)
}
