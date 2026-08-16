//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 206/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk206(t50: f64, t814: f64, t278: f64, t90: f64, t100: f64, t316: f64, t101: f64, t34: f64, t77: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t815 = t50 * t814;
    let t816 = -t278 + t815;
    let t817 = t90 * t816;
    let t820 = 1.0_f64 / t100;
    let t821 = t316 * t316;
    let t822 = t820 * t821;
    let t825 = -t816;
    let t826 = t101 * t825;
    let t830 = 1.0_f64 / t34 / t77;
    (t815, t816, t817, t820, t821, t822, t825, t826, t830)
}
