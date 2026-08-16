//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1008/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1008(t1259: f64, t13940: f64, t1256: f64, t1266: f64, t13035: f64, t13867: f64, t13869: f64, t13880: f64, t13884: f64, t13889: f64, t1657: f64, t3360: f64, t4490: f64, t4494: f64, t4517: f64, t538: f64, t5433: f64, t5449: f64) -> (f64, f64) {
    let t13941 = t1259 * t13940;
    let t13943 = -6.0_f64 * t1256 * t13880 + 4.0_f64 * t1256 * t13884 + 2.0_f64 * t1256 * t13889 - t1256 * t13941 - t1266 * t13869 - 2.0_f64 * t13035 * t1657 + t13867 * t538 + 2.0_f64 * t3360 * t5433 - t3360 * t5449 + 4.0_f64 * t4490 * t4494 - 2.0_f64 * t4490 * t4517;
    (t13941, t13943)
}
