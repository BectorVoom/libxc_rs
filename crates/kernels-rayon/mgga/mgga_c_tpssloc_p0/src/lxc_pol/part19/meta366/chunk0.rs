//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1332/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1332(t10283: f64, t969: f64, t10189: f64, t3014: f64, t2986: f64, t2990: f64, t10346: f64, t2987: f64, t10190: f64, t10245: f64, t10250: f64, t13779: f64) -> (f64, f64, f64, f64, f64) {
    let t42762 = t10283 * t969;
    let t42771 = t10189 * t3014;
    let t42773 = t2986 * t42771 * t2990;
    let t42775 = t2987 * t10346;
    let t42785 = t2986 * t10190 * t10245;
    let t42788 = t2986 * t13779 * t10250;
    (t42762, t42773, t42775, t42785, t42788)
}
