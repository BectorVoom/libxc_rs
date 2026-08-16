//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1023/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1023(t10668: f64, t531: f64, t10667: f64, t808: f64, t568: f64, t836: f64, t1628: f64, t3507: f64, t10019: f64, t2617: f64, t3005: f64, t7810: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11090 = t531 * t10668;
    let t11095 = t808 * t10667;
    let t11096 = t568 * t11095;
    let t11101 = t836 * t10667;
    let t11102 = t568 * t11101;
    let t11105 = t1628 * t3507;
    let t11108 = 0.15976219147466979032e-1_f64 * t10019;
    let t11109 = t3005 * t2617;
    let t11110 = t7810 * t11109;
    (t11090, t11095, t11096, t11101, t11102, t11105, t11108, t11109, t11110)
}
