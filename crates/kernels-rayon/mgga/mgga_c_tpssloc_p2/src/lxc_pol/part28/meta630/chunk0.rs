//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1973/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1973(t86950: f64, t86955: f64, t86967: f64, t225: f64, t26708: f64, t86991: f64, t13065: f64, t2054: f64, t24325: f64, t24330: f64, t26679: f64, t2718: f64, t4147: f64, t4268: f64, t47609: f64, t7092: f64, t82108: f64, t82115: f64, t82120: f64, t85060: f64, t855: f64, t865: f64, t86997: f64) -> (f64, f64, f64, f64, f64) {
    let t92431 = 0.15352717957250113407e0_f64 * t86950;
    let t92432 = 0.12793931631041761173e0_f64 * t86955;
    let t92434 = 0.15352717957250113407e0_f64 * t86967;
    let t92439 = t26708 * t225;
    let t92458 = 0.12793931631041761173e0_f64 * t86991;
    let t92464 = -2.0_f64 * t47609 * t2054 + 2.0_f64 * t4147 * t24330 + 4.0_f64 * t855 * t2718 * t26679 * t865 - 0.49348022005446793095e-1_f64 * t82108 + 4.0_f64 * t13065 * t7092 - t92458 + 4.0_f64 * t4268 * t24325 - 0.15352717957250113407e0_f64 * t82115 + 0.6579736267392905746e-1_f64 * t82120 - t85060 - 0.16449340668482264365e-1_f64 * t86997;
    (t92431, t92432, t92434, t92439, t92464)
}
