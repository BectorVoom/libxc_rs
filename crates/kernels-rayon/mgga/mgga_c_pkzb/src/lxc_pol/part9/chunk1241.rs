//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1241/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1241(t17946: f64, t21454: f64, t287: f64, t5726: f64, t2104: f64, t5974: f64, t7719: f64, t7649: f64, t2922: f64, t7654: f64, t774: f64, t7659: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21729 = t17946 * t21454;
    let t21730 = t5726 * t287;
    let t21746 = t2104 * t5974 * t7719;
    let t21749 = t2104 * t5974 * t7649;
    let t21752 = t2922 * t774 * t7654;
    let t21755 = t2922 * t774 * t7659;
    (t21729, t21730, t21746, t21749, t21752, t21755)
}
