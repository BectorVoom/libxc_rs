//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1286/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1286(t1317: f64, t21267: f64, t1324: f64, t11402: f64, t6957: f64, t1319: f64, t5481: f64, t5513: f64, t3820: f64, t6964: f64, t11491: f64, t5556: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21268 = t1317 * t21267;
    let t21270 = t1324 * t21267;
    let t21272 = t11402 * t6957;
    let t21273 = t21272 * t1319;
    let t21275 = t5513 * t5481;
    let t21277 = t3820 * t6964;
    let t21278 = t21277 * t1319;
    let t21280 = t11491 * t6957;
    let t21281 = t21280 * t1319;
    let t21283 = t5556 * t5481;
    (t21268, t21270, t21273, t21275, t21278, t21281, t21283)
}
