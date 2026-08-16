//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 792/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk792(t2739: f64, t875: f64, t840: f64, t871: f64, t2801: f64, t824: f64, t1882: f64, t2869: f64, t8232: f64, t837: f64, t10675: f64, t10678: f64, t10681: f64, t10685: f64, t10690: f64, t10693: f64, t10700: f64, t10705: f64, t10709: f64, t10714: f64, t10719: f64, t1901: f64, t446: f64) -> (f64, f64, f64, f64, f64) {
    let t10722 = t2739 * t875;
    let t10724 = t840 * t871 * t10722;
    let t10726 = t2801 * t824;
    let t10728 = t840 * t871 * t10726;
    let t10730 = t1882 * t2869;
    let t10732 = t8232 * t837;
    let t10734 = 2.0_f64 / 3.0_f64 * t446 * t10675 + t10678 / 9.0_f64 - t446 * t10681 - 2.0_f64 * t446 * t10685 + 2.0_f64 * t446 * t10690 + 2.0_f64 / 3.0_f64 * t10693 - 2.0_f64 * t446 * t10700 - 2.0_f64 / 3.0_f64 * t1901 * t10705 + 2.0_f64 * t446 * t10709 + 2.0_f64 * t446 * t10714 + 2.0_f64 * t446 * t10719 + t446 * t10724 + t446 * t10728 - 2.0_f64 / 3.0_f64 * t10730 - 4.0_f64 / 27.0_f64 * t10732;
    (t10722, t10724, t10726, t10728, t10734)
}
