//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1959/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1959(t21826: f64, t449: f64, t300: f64, t18910: f64, t4861: f64, t1164: f64, t4874: f64, t6085: f64, t1695: f64, t6084: f64, t1694: f64, t18615: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21827 = t21826 * t449;
    let t21829 = 0.19751673498613801407e-1_f64 * t300 * t21827;
    let t21830 = t18910 * t4861;
    let t21832 = 0.51947577317044391277e2_f64 * t1164 * t21830;
    let t21833 = t4874 * t6085;
    let t21835 = 0.35089341735807877242e1_f64 * t1164 * t21833;
    let t21836 = t1695 * t6084;
    let t21839 = t18615 * t1694;
    (t21827, t21829, t21830, t21832, t21833, t21835, t21836, t21839)
}
