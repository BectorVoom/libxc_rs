//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1286/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1286(t1115: f64, t3060: f64, t36967: f64, t3269: f64, t12739: f64, t42916: f64, t10610: f64, t11199: f64, t12414: f64, t12056: f64, t3275: f64, t7040: f64) -> (f64, f64, f64, f64) {
    let t45081 = t36967 * t1115 * t3060;
    let t45083 = 45.0_f64 / 64.0_f64 * t3269 * t45081;
    let t45085 = 3.0_f64 / 2.0_f64 * t42916 * t12739;
    let t45088 = 3.0_f64 / 2.0_f64 * t10610 * t11199 * t12414;
    let t45094 = t3275 * t12056 * t7040 / 2.0_f64;
    (t45083, t45085, t45088, t45094)
}
