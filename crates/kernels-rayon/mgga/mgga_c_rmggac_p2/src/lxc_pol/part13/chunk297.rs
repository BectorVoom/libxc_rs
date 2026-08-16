//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 297/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk297(t1528: f64, t196: f64, t1004: f64, t498: f64, t500: f64, t589: f64, t1022: f64, t1023: f64, t1050: f64, t1087: f64, t1094: f64, t1104: f64, t1112: f64, t1133: f64, t1140: f64, t1143: f64, t1424: f64, t1425: f64, t1429: f64, t1430: f64, t1434: f64, t1437: f64, t619: f64) -> (f64, f64, f64, f64) {
    let t1529 = t196 * t1528;
    let t1532 = t1004 * t498;
    let t1535 = t500 * t589;
    let t1538 = -t1424 + 0.93273e-1_f64 * t1425 * t1023 + t1429 + 0.186546e0_f64 * t1143 * t1430 - t1050 + 0.31091e-1_f64 * t1529 * t500 + t1133 - 0.31091e-1_f64 * t619 * t1532 - t1094 + t1104 + t1112 - t1087 - t1434 + t1140 + 0.93273e-1_f64 * t1022 * t1535 - t1437;
    (t1529, t1532, t1535, t1538)
}
