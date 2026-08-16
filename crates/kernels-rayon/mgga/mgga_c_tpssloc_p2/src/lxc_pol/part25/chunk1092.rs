//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1092/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1092(t3766: f64, t80827: f64, t1365: f64, t1878: f64, t12320: f64, t12426: f64, t22833: f64, t22813: f64, t6924: f64, t80782: f64, t22794: f64, t22843: f64, t281: f64, t6597: f64) -> (f64, f64, f64, f64, f64) {
    let t80828 = t80827 * t3766;
    let t80830 = t1878 * t1365;
    let t80831 = t80830 * t12320;
    let t80833 = t22833 * t12426;
    let t80836 = t22813 * t6924 * t80782;
    let t80837 = t80836 * t22794;
    let t80840 = t6597 * t22843 * t281;
    (t80828, t80831, t80833, t80837, t80840)
}
