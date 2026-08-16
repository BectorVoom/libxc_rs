//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2269/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2269(t11588: f64, t6144: f64, t3447: f64, t3451: f64, t15402: f64, t18237: f64, t1887: f64, t337: f64, t5416: f64, t4904: f64, t51968: f64, t1174: f64, t135: f64, t18525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t64779 = t11588 * t6144;
    let t64781 = t3447 * t64779 * t3451;
    let t64784 = t3447 * t15402 * t18237;
    let t64811 = t5416 * t337 * t1887;
    let t64821 = t3447 * t51968 * t4904;
    let t64858 = t1174 * t135 * t18525;
    (t64779, t64781, t64784, t64811, t64821, t64858)
}
