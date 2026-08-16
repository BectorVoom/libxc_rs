//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1074/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1074(t31136: f64, t31219: f64, t533: f64, t1390: f64, t1983: f64, t111: f64, t8312: f64) -> (f64, f64, f64, f64, f64) {
    let t31220 = t31136 + t31219;
    let t31221 = t533 * t31220;
    let t31222 = t31221 * t1390;
    let t31223 = t1983 * t31222;
    let t31224 = t8312 * t111;
    (t31220, t31221, t31222, t31223, t31224)
}
