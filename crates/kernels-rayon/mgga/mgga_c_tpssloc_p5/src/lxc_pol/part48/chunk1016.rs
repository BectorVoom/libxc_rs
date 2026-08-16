//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1016/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1016(t2098: f64, t7426: f64, t32392: f64, t580: f64, t32415: f64, t576: f64, t1404: f64, t8843: f64, t1395: f64, t8852: f64, t2105: f64, t7415: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t117407 = t2098 * t7426;
    let t117410 = t32392 * t580;
    let t117412 = t576 * t32415;
    let t117416 = t8843 * t1404;
    let t117418 = t1395 * t8852;
    let t117420 = t7415 * t2105;
    (t117407, t117410, t117412, t117416, t117418, t117420)
}
