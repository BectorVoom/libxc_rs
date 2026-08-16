//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1017/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1017(t10657: f64, t11566: f64, t11570: f64, t11580: f64, t11585: f64, t12382: f64, t12386: f64, t12388: f64, t12394: f64, t12398: f64, t12425: f64, t12432: f64, t12578: f64) -> f64 {
    let t12580 = -t12382 - 0.30487649791575028314e-3_f64 * t11566 + 0.43368970657079495312e-4_f64 * t11570 - t12386 + t12388 + t12394 + t12398 - t10657 + 0.19211284388664477842e-2_f64 * t11580 + 0.72042316457491791906e-3_f64 * t11585 + t12425 + t12432 + t12578;
    t12580
}
