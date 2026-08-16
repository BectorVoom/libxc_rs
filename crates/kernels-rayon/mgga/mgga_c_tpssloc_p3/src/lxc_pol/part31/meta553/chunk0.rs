//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1781/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1781(t2240: f64, t240: f64, t33: f64, t6492: f64, t23993: f64, t6495: f64, t1860: f64, t1864: f64, t67: f64, t835: f64, t6486: f64, t80743: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t84241 = t2240 * t33 * t240;
    let t84242 = t84241 * t6492;
    let t84248 = t6495 * t23993;
    let t84280 = 1232.0_f64 / 81.0_f64 * t1860 * t835 * t67 * t1864;
    let t84285 = t6486 * t23993;
    let t84400 = 0.3244175520728446583e0_f64 * t80743;
    (t84241, t84242, t84248, t84280, t84285, t84400)
}
