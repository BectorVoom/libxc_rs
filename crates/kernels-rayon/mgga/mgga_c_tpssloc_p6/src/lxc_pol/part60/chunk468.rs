//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 468/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk468(t291: f64, t5689: f64, t1557: f64, t4354: f64, t1556: f64, t913: f64, t2792: f64, t1547: f64, t2798: f64, t2802: f64, t4335: f64, t5679: f64, t5683: f64, t5687: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5691 = 0.621814e-1_f64 * t5689 * t291;
    let t5693 = 2.0_f64 * t4354 * t1557;
    let t5694 = t1556 * t1556;
    let t5695 = t5694 * t913;
    let t5697 = 2.0_f64 * t2792 * t5695;
    let t5698 = t1547 * t1547;
    let t5699 = t2798 * t5698;
    let t5705 = t2802 + 2.0_f64 / 9.0_f64 * t4335 - 2.0_f64 / 9.0_f64 * t5679 + 2.0_f64 / 3.0_f64 * t5683 - t5687 / 3.0_f64;
    (t5691, t5693, t5694, t5697, t5698, t5699, t5705)
}
