//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1121/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1121(t33185: f64, t8319: f64, t1873: f64, t7467: f64, t3941: f64, t5371: f64, t8326: f64, t1458: f64, t31267: f64, t33164: f64, t33177: f64, t33179: f64, t33181: f64, t33184: f64, t577: f64, t8508: f64) -> (f64, f64, f64, f64, f64) {
    let t33187 = 27.0_f64 * t33185 * t8319;
    let t33188 = t1873 * t7467;
    let t33190 = 54.0_f64 * t3941 * t33188;
    let t33191 = t5371 * t8326;
    let t33192 = 0.135e2_f64 * t33191;
    let t33193 = t8326 * t1458;
    let t33194 = t3941 * t33193;
    let t33195 = 27.0_f64 * t33194;
    let t33196 = 0.45e1_f64 * t33164 * t577 + 0.135e2_f64 * t31267 * t1458 + 27.0_f64 * t33177 + 54.0_f64 * t33179 + 27.0_f64 * t33181 + t33184 + t33187 + t33190 + t33192 + t33195 + t8508;
    (t33188, t33192, t33193, t33195, t33196)
}
