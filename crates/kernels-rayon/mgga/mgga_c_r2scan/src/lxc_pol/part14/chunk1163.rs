//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1163/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1163(t37982: f64, t7620: f64, t10856: f64, t7407: f64, t10868: f64, t2147: f64, t8066: f64, t7470: f64, t10708: f64, t7262: f64, t3281: f64, t10848: f64, t11760: f64, t2207: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40232 = t37982 * t7620;
    let t40234 = t10856 * t7407;
    let t40241 = t2147 * t10868 * t8066;
    let t40243 = t10856 * t7470;
    let t40251 = t10708 * t7262;
    let t40257 = t3281 * t7470;
    let t40260 = t2207 * t11760 * t10848;
    (t40232, t40234, t40241, t40243, t40251, t40257, t40260)
}
