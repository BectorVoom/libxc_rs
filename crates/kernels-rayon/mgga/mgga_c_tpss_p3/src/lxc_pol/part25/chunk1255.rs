//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1255/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1255(t1288: f64, t1692: f64, t1812: f64, t18728: f64, t18812: f64, t20514: f64, t21263: f64, t21266: f64, t21270: f64, t21353: f64, t21356: f64, t21359: f64, t21583: f64, t21659: f64, t2439: f64, t30: f64, t3552: f64, t4578: f64, t5853: f64, t6120: f64, t6153: f64, t6354: f64) -> f64 {
    let t21677 = 3.0_f64 * t3552 * t21583 + 3.0_f64 * t2439 * t6354 * t6120 - 3.0_f64 * t18728 * t21263 + 3.0_f64 * t2439 * t1812 * t21266 + 3.0_f64 / 2.0_f64 * t2439 * t1812 * t21270 + t1692 * t21659 * t30 / 2.0_f64 - t1692 * t20514 * t6153 + t1692 * t6354 * t1288 + t1692 * t18812 * t21353 - t1692 * t5853 * t21356 - t1692 * t5853 * t21359 / 2.0_f64 + t1692 * t1812 * t4578 / 2.0_f64;
    t21677
}
