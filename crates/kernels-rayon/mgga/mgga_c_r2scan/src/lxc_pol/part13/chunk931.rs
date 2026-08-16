//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 931/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk931(t10710: f64, t6481: f64, t10708: f64, t1584: f64, t3309: f64, t2124: f64, t5173: f64, t3295: f64, t3308: f64, t6536: f64, t1577: f64, t6166: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10711 = t10710 * t6481;
    let t10712 = t10708 * t10711;
    let t10713 = 0.14282990759302185292e-1_f64 * t10712;
    let t10714 = t1584 * t3309;
    let t10716 = t2124 * t5173;
    let t10717 = t3295 * t10716;
    let t10719 = t3308 * t6536;
    let t10720 = t1577 * t10719;
    let t10722 = t3308 * t6166;
    (t10711, t10712, t10713, t10714, t10716, t10717, t10719, t10720, t10722)
}
