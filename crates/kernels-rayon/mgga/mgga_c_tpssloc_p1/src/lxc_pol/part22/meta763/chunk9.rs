//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2579/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2579(t1703: f64, t65288: f64, t71543: f64, t71545: f64, t71547: f64, t71655: f64, t71657: f64, t71697: f64, t72061: f64, t72065: f64, t72067: f64, t72071: f64) -> (f64, f64) {
    let t72073 = 0.17544670867903938621e1_f64 * t65288 * t1703;
    let t72074 = -t72061 - t72065 + t71543 - t71545 + t71547 + t71655 + t71657 + t72067 - t72071 - t72073 - t71697;
    (t72073, t72074)
}
