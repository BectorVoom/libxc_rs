//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1030/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1030(t12943: f64, t12391: f64, t3465: f64, t3262: f64, t11506: f64, t12210: f64, t12395: f64, t3472: f64, t3275: f64, t9573: f64, t12056: f64, t2867: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12944 = 5.0_f64 / 8.0_f64 * t12943;
    let t12945 = t3465 * t12391;
    let t12946 = t3262 * t12945;
    let t12947 = 3.0_f64 / 2.0_f64 * t12946;
    let t12948 = t11506 * t12210;
    let t12949 = 3.0_f64 / 2.0_f64 * t12948;
    let t12951 = t3472 * t12395;
    let t12952 = t3262 * t12951;
    let t12953 = 15.0_f64 / 8.0_f64 * t12952;
    let t12957 = t3275 * t3465 * t9573;
    let t12958 = t12957 / 2.0_f64;
    let t12960 = t3275 * t12056 * t2867;
    (t12944, t12945, t12947, t12949, t12951, t12953, t12958, t12960)
}
