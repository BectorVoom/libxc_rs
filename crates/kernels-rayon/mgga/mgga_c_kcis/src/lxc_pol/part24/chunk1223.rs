//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1223/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1223(t19940: f64, t28029: f64, t1808: f64, t26929: f64, t5182: f64, t6720: f64, t92515: f64, t1189: f64, t18459: f64, t26933: f64, t6693: f64, t28059: f64, t5091: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99945 = t28029 * t19940;
    let t99948 = t1808 * t26929 * t5182;
    let t99950 = t92515 * t6720;
    let t99952 = t18459 * t1189;
    let t99954 = t26933 * t6693;
    let t99956 = t28059 * t5091;
    (t99945, t99948, t99950, t99952, t99954, t99956)
}
