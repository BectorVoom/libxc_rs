//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 638/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk638(t1462: f64, t236: f64, t1971: f64, t8517: f64, t2344: f64, t7494: f64, t1587: f64, t649: f64, t27: f64, t2134: f64, t2329: f64, t7501: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8518 = t236 * t1462;
    let t8519 = t1971 * t8518;
    let t8520 = t8517 * t8519;
    let t8523 = t7494 * t2344;
    let t8525 = t649 * t1587;
    let t8526 = t27 * t8525;
    let t8527 = t2134 * t8526;
    let t8529 = t7501 * t2329;
    (t8519, t8520, t8523, t8526, t8527, t8529)
}
