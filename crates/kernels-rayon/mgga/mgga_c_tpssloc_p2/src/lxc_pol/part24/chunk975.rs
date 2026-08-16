//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 975/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk975(t10622: f64, t10649: f64, t10652: f64, t10654: f64, t10657: f64, t10665: f64, t1068: f64, t10699: f64, t10707: f64, t10715: f64, t10739: f64, t10819: f64, t10855: f64, t3209: f64, t3216: f64, t4700: f64) -> f64 {
    let t11103 = -3.0_f64 * t1068 * t3209 * t3216 * t4700 + t10622 - t10649 + t10652 + t10654 + t10657 - t10665 + t10699 + t10707 + t10715 + t10739 - t10819 - t10855;
    t11103
}
