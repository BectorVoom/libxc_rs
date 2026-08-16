//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 586/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk586(t2004: f64, t7720: f64, t2007: f64, t1987: f64, t1990: f64, t333: f64, t495: f64, t511: f64, t1971: f64, t7230: f64, t498: f64, t7231: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7721 = t7720 * t2004;
    let t7723 = t7720 * t2007;
    let t7725 = t7720 * t1987;
    let t7727 = t7720 * t1990;
    let t7731 = t333 * t495;
    let t7732 = t511 * t7731;
    let t7733 = t1971 * t7732;
    let t7734 = t7230 * t7733;
    let t7737 = t511 * t333 * t498;
    let t7738 = t7231 * t7737;
    (t7721, t7723, t7725, t7727, t7733, t7734, t7738)
}
