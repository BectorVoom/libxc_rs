//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 229/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk229(t1334: f64, t1335: f64, t1316: f64, t1305: f64, t1309: f64) -> (f64, f64, f64, f64) {
    let t1336 = t1334 * t1335;
    let t1338 = 1.0_f64 * t1316 * t1336;
    let t1339 = 0.92708333333333333333e-2_f64 * t1305;
    let t1341 = -t1339 - 0.92708333333333333333e-2_f64 * t1309;
    (t1336, t1338, t1339, t1341)
}
