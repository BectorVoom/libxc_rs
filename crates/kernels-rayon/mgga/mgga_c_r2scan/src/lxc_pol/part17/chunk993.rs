//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 993/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk993(t12123: f64, t12133: f64, t12146: f64, t12159: f64, t12168: f64, t12180: f64, t12188: f64, t12194: f64, t797: f64, t1048: f64, t499: f64, t11002: f64, t1115: f64, t983: f64) -> (f64, f64, f64, f64, f64) {
    let t12197 = t12123 + t12133 + t12146 + t12159 + t12168 + t12180 + t12188 + t12194;
    let t12198 = t12197 * t797;
    let t12200 = t1048 * t499 * t12198;
    let t12201 = t12200 / 4.0_f64;
    let t12203 = t11002 * t1115 * t983;
    (t12197, t12198, t12200, t12201, t12203)
}
