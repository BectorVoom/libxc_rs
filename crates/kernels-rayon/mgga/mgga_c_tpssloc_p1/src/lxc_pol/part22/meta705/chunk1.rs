//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2295/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2295(t1244: f64, t3068: f64, t478: f64, t6163: f64, t11697: f64, t18386: f64, t3577: f64, t15608: f64, t15740: f64, t1174: f64, t6183: f64, t698: f64) -> (f64, f64, f64, f64) {
    let t66622 = t1244 * t478 * t6163 * t3068;
    let t66646 = t3577 * t11697 * t18386;
    let t66648 = t15740 * t15608;
    let t66668 = t1174 * t698 * t6183;
    (t66622, t66646, t66648, t66668)
}
