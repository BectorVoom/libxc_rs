//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1972/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1972(t22690: f64, t3787: f64, t22832: f64, t3777: f64, t1336: f64, t6943: f64, t836: f64, t1995: f64, t1999: f64, t213: f64, t39041: f64, t557: f64, t6546: f64) -> (f64, f64, f64, f64, f64) {
    let t80798 = t22690 * t3787;
    let t80816 = t3777 * t22832;
    let t80820 = t1336 * t6943 * t836;
    let t80825 = t39041 * t1995 * t213 * t1999;
    let t80826 = 0.10173934535723378495e0_f64 * t80825;
    let t80827 = t6546 * t557;
    (t80798, t80816, t80820, t80826, t80827)
}
