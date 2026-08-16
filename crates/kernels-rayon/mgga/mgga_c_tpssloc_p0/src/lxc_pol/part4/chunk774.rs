//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 774/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk774(t225: f64, t5600: f64, t2671: f64, t5527: f64, t5544: f64, t824: f64, t1504: f64, t1506: f64, t228: f64, t230: f64, t232: f64, t819: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5601 = t5600 * t225;
    let t5605 = t2671 * t5527;
    let t5608 = t824 * t5544;
    let t5611 = 6.0_f64 * t1504 * t1506 - 12.0_f64 * t228 * t5605 + 3.0_f64 * t228 * t5608 - t230 * t5601;
    let t5612 = t5611 * t232;
    let t5614 = t819 * t820 * t5612;
    (t5601, t5605, t5608, t5611, t5612, t5614)
}
