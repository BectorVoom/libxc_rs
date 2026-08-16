//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1325/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1325(t7540: f64, t868: f64, t25373: f64, t118413: f64, t25927: f64, t118466: f64, t23788: f64, t1081: f64, t1649: f64, t6665: f64, t25353: f64, t28: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t118953 = t7540 * t868;
    let t118954 = t25373 * t118953;
    let t119700 = t25927 * t118413;
    let t119719 = t23788 * t118466;
    let t119737 = t1081 * t7540;
    let t119743 = t25927 * t118953;
    let t119746 = t1649 * t6665;
    let t119766 = t28 * t25353;
    (t118953, t118954, t119700, t119719, t119737, t119743, t119746, t119766)
}
