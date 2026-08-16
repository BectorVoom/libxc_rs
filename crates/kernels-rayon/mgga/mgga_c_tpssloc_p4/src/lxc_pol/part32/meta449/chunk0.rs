//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1712/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1712(t1329: f64, t22797: f64, t2230: f64, t6924: f64, t213: f64, t6928: f64, t10: f64, t2229: f64, t60: f64, t1995: f64, t116: f64, t117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22798 = t22797 * t1329;
    let t22803 = t2230 * t6924;
    let t22804 = t22803 * t213;
    let t22805 = t22804 * t6928;
    let t22811 = t2229 * t10;
    let t22813 = 1.0_f64 / t60 / t22811;
    let t22814 = t22813 * t1995;
    let t22815 = t117 * t116;
    (t22798, t22803, t22804, t22805, t22811, t22813, t22814, t22815)
}
