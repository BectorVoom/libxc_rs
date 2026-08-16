//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1085/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1085(t10869: f64, t6395: f64, t20421: f64, t6162: f64, t6327: f64, t10833: f64, t776: f64, t1615: f64, t269: f64, t2147: f64, t2150: f64, t507: f64, t512: f64, t6100: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38161 = t6395 * t10869;
    let t38164 = t6327 * t20421 * t6162;
    let t38165 = 0.25705033881751801528e-4_f64 * t38164;
    let t38166 = t776 * t10833;
    let t38168 = t1615 * t269;
    let t38170 = t2147 * t38168 * t2150;
    let t38175 = t512 * t6100 * t507;
    (t38161, t38165, t38166, t38168, t38170, t38175)
}
