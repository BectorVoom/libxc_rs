//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 500/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk500(t99: f64, t107: f64, t680: f64, t691: f64, t205: f64, t256: f64, t21: f64, t65: f64, t64: f64, t159: f64, t216: f64, t756: f64, t760: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2083 = 1.0_f64 / t99;
    let t2091 = 1.0_f64 / t107;
    let t2112 = t680 * t691;
    let t2115 = t205 * t256;
    let t2138 = 1.0_f64 / t65 / t21;
    let t2139 = t64 * t2138;
    let t2140 = t2139 * t159;
    let t2142 = 35.0_f64 / 432.0_f64 * t2140 * t216;
    let t2143 = t756 * t760;
    (t2083, t2091, t2112, t2115, t2138, t2139, t2140, t2142, t2143)
}
