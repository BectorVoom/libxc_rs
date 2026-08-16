//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 259/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk259(t53: f64, t1368: f64, t352: f64, t171: f64, t577: f64, t433: f64, t521: f64, t983: f64, t437: f64, t50: f64, t280: f64, t814: f64, t525: f64, t990: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t1369 = t1368 * t352;
    let t1372 = t577 * t171;
    let t1373 = t1372 * t433;
    let t1374 = 0.5848223622634646207e0_f64 * t1373;
    let t1375 = t983 * t521;
    let t1378 = t437 * t50;
    let t1382 = piecewise3(t54, 0.0_f64, -2.0_f64 / 9.0_f64 * t1375 * t280 + 4.0_f64 / 3.0_f64 * t1378 * t814);
    let t1383 = t990 * t525;
    (t1369, t1372, t1373, t1374, t1375, t1382, t1383)
}
