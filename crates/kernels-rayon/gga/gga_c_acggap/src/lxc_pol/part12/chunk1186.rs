//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1186/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1186(t35039: f64, t35041: f64, t35043: f64, t35051: f64, t35055: f64, t35070: f64, t35072: f64, t35074: f64, t35076: f64, t30916: f64, t30918: f64, t32635: f64, t35047: f64, t35059: f64, t35062: f64, t35065: f64, t35068: f64, t35080: f64) -> f64 {
    let t37361 = 7.0_f64 / 36.0_f64 * t35039;
    let t37362 = 7.0_f64 / 36.0_f64 * t35041;
    let t37363 = 35.0_f64 / 108.0_f64 * t35043;
    let t37365 = 0.28582678745379824648e-3_f64 * t35051;
    let t37366 = 0.15724046144802076034e-2_f64 * t35055;
    let t37372 = 0.16809375e0_f64 * t35070;
    let t37373 = 0.16809375e0_f64 * t35072;
    let t37374 = 0.1120625e0_f64 * t35074;
    let t37375 = 77.0_f64 / 288.0_f64 * t35076;
    let t37377 = 0.17149607247227894789e-2_f64 * t30916 + t37361 + t37362 - t37363 - 0.21437009059034868486e-3_f64 * t35047 - t37365 - t37366 + 0.94344276868812456207e-3_f64 * t30918 - t35059 / 8.0_f64 - t35062 / 8.0_f64 - 0.4584375e-1_f64 * t35065 - 0.916875e-1_f64 * t35068 - t37372 - t37373 - t37374 - t32635 - t37375 - 0.7640625e-2_f64 * t35080;
    t37377
}
