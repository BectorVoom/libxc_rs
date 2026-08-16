//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 886/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk886(t2146: f64, t756: f64, t159: f64, t799: f64, t210: f64, t2139: f64, t760: f64, t764: f64, t64: f64, t7091: f64, t216: f64, t570: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8167 = t756 * t2146;
    let t8170 = t159 * t799;
    let t8171 = t210 * t8170;
    let t8176 = t2139 * t760;
    let t8177 = t8176 * t764;
    let t8185 = t64 * t7091;
    let t8186 = t8185 * t159;
    let t8188 = 455.0_f64 / 1296.0_f64 * t8186 * t216;
    let t8199 = 1.0_f64 / t66 / t570;
    (t8167, t8171, t8176, t8177, t8186, t8188, t8199)
}
