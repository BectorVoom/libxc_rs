//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 598/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk598(t640: f64, t7352: f64, t7764: f64, t2019: f64, t2064: f64, t333: f64, t903: f64, t665: f64, t839: f64, t1364: f64, t794: f64, t1550: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7765 = t640 * t7352;
    let t7766 = t7764 * t7765;
    let t7767 = t2019 * t7766;
    let t7769 = t2064 * t333;
    let t7770 = t903 * t7769;
    let t7772 = t665 * t839;
    let t7773 = t1364 * t7772;
    let t7774 = 0.23948483403727617128e0_f64 * t7773;
    let t7775 = t665 * t794;
    let t7776 = t1550 * t7775;
    (t7765, t7766, t7767, t7769, t7770, t7772, t7774, t7775, t7776)
}
