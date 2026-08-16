//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 545/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk545(t3211: f64, t779: f64, t3276: f64, t740: f64, t3234: f64, t795: f64, t835: f64, t723: f64, t2580: f64, t2089: f64, t3209: f64, t7226: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9719 = t779 * t3211;
    let t9722 = t3276 * t740;
    let t9725 = t795 * t3234;
    let t9726 = t9725 * t740;
    let t9729 = t835 * t3234;
    let t9730 = t9729 * t723;
    let t9731 = t2580 * t9730;
    let t9734 = t2089 * t3209;
    let t9735 = t9734 * t723;
    let t9736 = t7226 * t9735;
    (t9719, t9722, t9725, t9726, t9729, t9730, t9731, t9734, t9735, t9736)
}
