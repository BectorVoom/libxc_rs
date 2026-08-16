//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1150/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1150(t167: f64, t1717: f64, t4670: f64, t4836: f64, t13677: f64, t1727: f64, t6313: f64, t829: f64, t1035: f64, t6276: f64, t1045: f64, t6317: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19347 = t1717 * t167;
    let t19350 = t4836 * t4670;
    let t19353 = t13677 * t1727;
    let t19356 = t6313 * t829;
    let t19359 = t1035 * t6276;
    let t19360 = t19359 * t1045;
    let t19363 = t6317 * t829;
    (t19347, t19350, t19353, t19356, t19360, t19363)
}
