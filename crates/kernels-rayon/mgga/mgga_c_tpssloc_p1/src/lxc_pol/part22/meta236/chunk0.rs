//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1315/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1315(t153: f64, t9862: f64, t2371: f64, t2531: f64, t2528: f64, t2517: f64, t607: f64, t707: f64, t2652: f64, t2663: f64, t181: f64, t686: f64, t781: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9863 = t153 * t9862;
    let t9864 = t2531 * t2371;
    let t9866 = t2531 * t2528;
    let t9868 = t2517 * t607;
    let t9869 = t707 * t9868;
    let t9871 = t2652 * t2663;
    let t9874 = t686 * t781 * t181;
    (t9863, t9864, t9866, t9868, t9869, t9871, t9874)
}
