//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 834/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk834(t2558: f64, t36798: f64, t9647: f64, t10697: f64, t10742: f64, t11662: f64, t2554: f64, t7064: f64, t35611: f64, t5539: f64, t42973: f64, t2581: f64, t550: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44755 = t9647 * t36798 * t2558;
    let t44756 = 0.32043859292259267849e-3_f64 * t44755;
    let t44758 = t9647 * t10697 * t10742;
    let t44759 = 0.19226315575355560709e-2_f64 * t44758;
    let t44761 = t7064 * t11662 * t2554;
    let t44762 = 0.32043859292259267849e-3_f64 * t44761;
    let t44764 = t9647 * t5539 * t35611;
    let t44776 = 0.1281754371690370714e-2_f64 * t42973;
    let t44777 = t550 * t2581;
    (t44756, t44759, t44762, t44764, t44776, t44777)
}
