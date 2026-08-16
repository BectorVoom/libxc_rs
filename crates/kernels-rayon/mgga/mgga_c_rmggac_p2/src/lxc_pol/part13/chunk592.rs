//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 592/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk592(t640: f64, t7352: f64, t7764: f64, t2019: f64, t2064: f64, t333: f64, t903: f64, t265: f64, t338: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7765 = t640 * t7352;
    let t7766 = t7764 * t7765;
    let t7767 = t2019 * t7766;
    let t7769 = t2064 * t333;
    let t7770 = t903 * t7769;
    let t7778 = t338 * t265;
    (t7765, t7766, t7767, t7769, t7770, t7778)
}
