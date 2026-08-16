//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 660/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk660(t124: f64, t383: f64, t402: f64, t625: f64, t1460: f64, t1478: f64, t1482: f64, t377: f64, t1486: f64, t1465: f64, t1468: f64, t1497: f64) -> (f64, f64, f64, f64, f64) {
    let t4788 = t124 * t383;
    let t4790 = t625 * t4788 * t402;
    let t4791 = 0.71233333333333333332e-1_f64 * t4790;
    let t4793 = t625 * t1460 * t1478;
    let t4794 = 0.53424999999999999999e-1_f64 * t4793;
    let t4795 = t377 * t1482;
    let t4797 = t625 * t4795 * t1486;
    let t4798 = 0.85917975471764868594e0_f64 * t4797;
    let t4805 = t625 * t377 * t1465 * t1468;
    let t4806 = 0.10685e0_f64 * t4805;
    let t4807 = t377 * t1497;
    (t4791, t4794, t4798, t4806, t4807)
}
