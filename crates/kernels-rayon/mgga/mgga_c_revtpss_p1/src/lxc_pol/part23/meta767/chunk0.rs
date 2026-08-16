//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2567/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2567(t12904: f64, t5274: f64, t11262: f64, t1261: f64, t5303: f64, t3711: f64, t5298: f64, t127: f64, t17352: f64, t5293: f64, t5269: f64, t3140: f64, t5216: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t56726 = t5274 * t12904;
    let t56727 = 0.14291339372689912324e-3_f64 * t56726;
    let t56739 = t1261 * t11262 * t5303;
    let t56740 = 0.15879265969655458138e-3_f64 * t56739;
    let t56742 = t3711 * t11262 * t5298;
    let t56756 = t127 * t17352;
    let t56785 = t5293 * t12904;
    let t56786 = 0.7622047665434619906e-3_f64 * t56785;
    let t56790 = t1261 * t11262 * t5269;
    let t56791 = 0.19055119163586549765e-3_f64 * t56790;
    let t56802 = t5216 * t3140;
    (t56727, t56740, t56742, t56756, t56786, t56791, t56802)
}
