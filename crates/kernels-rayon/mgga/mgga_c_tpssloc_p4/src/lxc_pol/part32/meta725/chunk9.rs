//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2338/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2338(t225: f64, t29687: f64, t104453: f64, t1252: f64, t1721: f64, t1761: f64, t2155: f64, t254: f64, t27396: f64, t27406: f64, t27549: f64, t27742: f64, t27761: f64, t27767: f64, t27775: f64, t27779: f64, t27786: f64, t29532: f64, t3593: f64, t466: f64, t498: f64, t5055: f64, t65208: f64, t7999: f64, t94514: f64, t94779: f64, t95824: f64, t95902: f64) -> f64 {
    let t104556 = t29687 * t225;
    let t104564 = -12.0_f64 * t1721 * t254 * t27786 + 4.0_f64 * t3593 * t29532 + 4.0_f64 * t5055 * t27396 + 0.14621636149762012769e-1_f64 * t27406 * t27779 - 0.43864908449286038306e-1_f64 * t7999 * t27767 + 4.0_f64 * t5055 * t27761 - 2.0_f64 * t5055 * t27742 - t65208 * t2155 - 0.73108180748810063845e-2_f64 * t27549 * t94514 * t27775 - t94779 - 2.0_f64 * t104556 * t1252 + 0.97477574331746751793e-2_f64 * t95824 - 2.0_f64 * t95902 * t1761 + t466 * t104453 * t498;
    t104564
}
