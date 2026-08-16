//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1237/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1237(t10995: f64, t7771: f64, t27042: f64, t27055: f64, t27014: f64, t27023: f64, t2193: f64, t2196: f64, t44682: f64, t26982: f64, t7784: f64, t1014: f64, t26840: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92948 = t7771 * t10995;
    let t92951 = t27042 * t27055;
    let t92955 = t27014 * t27023;
    let t92958 = t27014 * t27055;
    let t92964 = 0.12871334876543209877e-3_f64 * t2193 * t44682 * t2196;
    let t92976 = t26982 * t7784;
    let t92981 = t1014 * t26840;
    (t92948, t92951, t92955, t92958, t92964, t92976, t92981)
}
