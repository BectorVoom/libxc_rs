//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 897/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk897(t9798: f64, t9860: f64, t157: f64, t153: f64, t2371: f64, t2531: f64, t2528: f64, t2517: f64, t607: f64, t707: f64, t2652: f64, t2663: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9861 = t9798 + t9860;
    let t9862 = t157 * t9861;
    let t9863 = t153 * t9862;
    let t9864 = t2531 * t2371;
    let t9866 = t2531 * t2528;
    let t9868 = t2517 * t607;
    let t9869 = t707 * t9868;
    let t9871 = t2652 * t2663;
    (t9861, t9863, t9864, t9866, t9869, t9871)
}
