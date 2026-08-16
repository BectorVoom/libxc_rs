//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 282/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk282(t333: f64, t559: f64, t338: f64, t558: f64, t352: f64, t171: f64, t577: f64, t433: f64, t521: f64, t983: f64, t437: f64, t50: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1365 = t559 * t333;
    let t1368 = t338 * t558;
    let t1369 = t1368 * t352;
    let t1372 = t577 * t171;
    let t1373 = t1372 * t433;
    let t1374 = 0.5848223622634646207e0_f64 * t1373;
    let t1375 = t983 * t521;
    let t1378 = t437 * t50;
    (t1365, t1368, t1369, t1372, t1374, t1375, t1378)
}
