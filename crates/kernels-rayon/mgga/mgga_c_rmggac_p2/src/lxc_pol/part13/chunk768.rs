//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 768/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk768(t7338: f64, t7345: f64, t7341: f64, t4905: f64, t7778: f64, t903: f64, t2064: f64, t833: f64, t1550: f64, t1338: f64, t2039: f64, t357: f64, t638: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35742 = t7345 * t7338;
    let t35744 = t7345 * t7341;
    let t35752 = t903 * t7778 * t4905;
    let t35765 = t2064 * t833;
    let t35766 = t1550 * t35765;
    let t35772 = t638 * t2039 * t357 * t1338;
    (t35742, t35744, t35752, t35765, t35766, t35772)
}
