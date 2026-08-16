//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 982/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk982(t114866: f64, t6552: f64, t7479: f64, t25341: f64, t31366: f64, t1880: f64, t26679: f64, t6553: f64, t6571: f64, t22986: f64, t23270: f64, t31332: f64, t98960: f64) -> (f64, f64, f64, f64) {
    let t121311 = t6552 * t114866 * t7479;
    let t121314 = t6552 * t31366 * t25341;
    let t121318 = t1880 * t6553 * t6571 * t26679;
    let t121326 = t22986 * t23270 * t31332 * t98960;
    (t121311, t121314, t121318, t121326)
}
