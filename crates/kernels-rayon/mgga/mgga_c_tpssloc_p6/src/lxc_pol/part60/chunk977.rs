//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 977/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk977(t32673: f64, t32675: f64, t32678: f64, t1845: f64, t7752: f64, t120179: f64, t1992: f64, t32693: f64, t90566: f64, t22635: f64, t31090: f64, t6460: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127122 = 4.0_f64 * t32673;
    let t127124 = 4.0_f64 * t32675;
    let t127125 = 4.0_f64 * t32678;
    let t127162 = t1845 * t7752;
    let t127166 = 0.15352717957250113407e0_f64 * t120179;
    let t127169 = 0.6579736267392905746e-1_f64 * t1992 * t90566 * t32693;
    let t127173 = 0.3289868133696452873e-1_f64 * t1992 * t22635 * t31090 * t6460;
    (t127122, t127124, t127125, t127162, t127166, t127169, t127173)
}
