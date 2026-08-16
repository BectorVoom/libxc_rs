//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 906/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk906(t3448: f64, t6138: f64, t6144: f64, t11583: f64, t5392: f64, t15338: f64, t4904: f64, t3447: f64, t3431: f64, t6126: f64, t1174: f64, t6130: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18416 = t3448 * t6138;
    let t18420 = t3448 * t6144;
    let t18427 = t11583 * t5392;
    let t18446 = t15338 * t4904;
    let t18447 = t3447 * t18446;
    let t18451 = t3431 * t6126;
    let t18452 = t1174 * t18451;
    let t18454 = t3431 * t6130;
    (t18416, t18420, t18427, t18446, t18447, t18451, t18452, t18454)
}
