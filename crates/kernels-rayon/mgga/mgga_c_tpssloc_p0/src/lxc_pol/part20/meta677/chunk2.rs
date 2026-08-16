//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2560/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2560(t51441: f64, t51443: f64, t51446: f64, t51449: f64, t51453: f64, t51456: f64, t51459: f64, t51463: f64, t51466: f64, t51470: f64, t51472: f64, t11433: f64, t1164: f64, t14966: f64) -> (f64, f64) {
    let t51826 = t51441 + t51443 - t51446 - t51449 - t51453 - t51456 + t51459 + t51463 + t51466 + t51470 - t51472;
    let t51831 = 0.51947577317044391277e2_f64 * t1164 * t14966 * t11433;
    (t51826, t51831)
}
