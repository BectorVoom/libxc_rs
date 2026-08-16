//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1960/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1960(t1683: f64, t6052: f64, t1682: f64, t18643: f64, t6036: f64, t3359: f64, t11314: f64, t11317: f64, t14702: f64, t14766: f64, t18203: f64, t18219: f64, t18229: f64, t18494: f64, t18505: f64, t18512: f64, t21739: f64, t21741: f64, t21747: f64, t21751: f64) -> (f64, f64, f64, f64, f64) {
    let t21842 = t1683 * t6052;
    let t21845 = t18643 * t1682;
    let t21854 = t6036 * t1682;
    let t21855 = t21854 * t3359;
    let t21870 = -t11314 - 0.20839e0_f64 * t18512 + 0.34431666666666666666e0_f64 * t18203 - 0.103295e1_f64 * t18219 - 0.51647499999999999999e0_f64 * t18229 + 0.69463333333333333335e-1_f64 * t18494 - 0.41678000000000000001e0_f64 * t18505 - 0.52945875e1_f64 * t21739 + 0.94674375e0_f64 * t21741 - t11317 + 0.68863333333333333332e0_f64 * t14702 + 0.34731666666666666667e0_f64 * t14766 - 0.104195e0_f64 * t21747 + 0.62517e0_f64 * t21751;
    (t21842, t21845, t21854, t21855, t21870)
}
