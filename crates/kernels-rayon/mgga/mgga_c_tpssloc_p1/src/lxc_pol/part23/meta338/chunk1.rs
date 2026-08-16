//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1112/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1112(t2369: f64, t9720: f64, t9843: f64, t1294: f64, t2411: f64, t2414: f64, t39246: f64) -> (f64, f64, f64) {
    let t39362 = t9720 * t2369 * t9843;
    let t39364 = 0.62337092780453269531e3_f64 * t1294 * t39362;
    let t39373 = 0.48245938496077605201e2_f64 * t2411 * t39246 * t2414;
    (t39362, t39364, t39373)
}
