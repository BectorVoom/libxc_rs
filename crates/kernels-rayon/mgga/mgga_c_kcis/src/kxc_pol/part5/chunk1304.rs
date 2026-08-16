//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1304/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1304(t18431: f64, t518: f64, t1319: f64, t6957: f64, t1897: f64, t5481: f64, t6964: f64, t1317: f64, t7138: f64, t21186: f64, t21188: f64, t21193: f64, t21196: f64, t21206: f64, t21209: f64, t21212: f64, t21234: f64, t21237: f64, t21240: f64, t21243: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21534 = t518 * t18431;
    let t21537 = t6957 * t1319;
    let t21542 = t1897 * t5481;
    let t21551 = t6964 * t1319;
    let t21558 = t1317 * t7138;
    let t21581 = 0.91722222222222222223e-3_f64 * t21186 - 0.45861111111111111112e-2_f64 * t21237 + 0.1651e-1_f64 * t21234 + 0.11006666666666666667e-1_f64 * t21240 - 0.27516666666666666667e-2_f64 * t21188 - 0.24765e-1_f64 * t21243 - 0.3302e-1_f64 * t21206 + 0.13758333333333333333e-2_f64 * t21196 - 0.27516666666666666667e-2_f64 * t21209 + 0.8255e-2_f64 * t21212 - 0.41275e-2_f64 * t21193;
    (t21534, t21537, t21542, t21551, t21558, t21581)
}
