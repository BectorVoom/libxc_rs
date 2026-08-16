//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1314/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1314(t10767: f64, t5552: f64, t17974: f64, t3685: f64, t10664: f64, t5559: f64, t10669: f64, t10662: f64, t19696: f64, t215: f64, t10667: f64, t19695: f64, t19697: f64, t5543: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t63975 = t5552 * t10767;
    let t63977 = t17974 * t3685;
    let t63979 = t5559 * t10664;
    let t63981 = t5559 * t10669;
    let t63984 = t19696 * t215 * t10662;
    let t63987 = t19696 * t215 * t10667;
    let t63990 = t5543 * t19695 * t19697;
    (t63975, t63977, t63979, t63981, t63984, t63987, t63990)
}
