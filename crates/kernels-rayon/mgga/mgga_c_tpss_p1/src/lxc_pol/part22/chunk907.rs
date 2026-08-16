//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 907/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk907(t2149: f64, t8167: f64, t159: f64, t799: f64, t210: f64, t2139: f64, t760: f64, t764: f64, t2143: f64, t2153: f64, t64: f64, t7091: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8168 = t8167 * t2149;
    let t8170 = t159 * t799;
    let t8171 = t210 * t8170;
    let t8176 = t2139 * t760;
    let t8177 = t8176 * t764;
    let t8179 = t2143 * t2153;
    let t8185 = t64 * t7091;
    (t8168, t8171, t8176, t8177, t8179, t8185)
}
