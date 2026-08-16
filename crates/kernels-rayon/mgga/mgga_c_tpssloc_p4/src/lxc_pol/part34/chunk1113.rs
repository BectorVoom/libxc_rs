//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1113/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1113(t81437: f64, t39063: f64, t7025: f64, t23966: f64, t9239: f64, t2240: f64, t240: f64, t33: f64, t1860: f64, t1864: f64, t67: f64, t835: f64) -> (f64, f64, f64, f64, f64) {
    let t84036 = 308.0_f64 / 27.0_f64 * t81437;
    let t84216 = t39063 * t7025;
    let t84219 = t9239 * t23966;
    let t84241 = t2240 * t33 * t240;
    let t84280 = 1232.0_f64 / 81.0_f64 * t1860 * t835 * t67 * t1864;
    (t84036, t84216, t84219, t84241, t84280)
}
