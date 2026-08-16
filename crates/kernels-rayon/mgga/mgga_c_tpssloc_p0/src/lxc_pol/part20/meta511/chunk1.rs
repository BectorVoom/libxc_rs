//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2028/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2028(t1294: f64, t39362: f64, t3814: f64, t9874: f64, t1307: f64, t3914: f64, t2411: f64, t2414: f64, t39246: f64) -> (f64, f64, f64, f64) {
    let t39364 = 0.62337092780453269531e3_f64 * t1294 * t39362;
    let t39365 = t3814 * t9874;
    let t39367 = t1307 * t3914;
    let t39373 = 0.48245938496077605201e2_f64 * t2411 * t39246 * t2414;
    (t39364, t39365, t39367, t39373)
}
