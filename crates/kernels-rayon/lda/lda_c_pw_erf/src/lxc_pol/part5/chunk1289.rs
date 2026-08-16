//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1289/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1289(t18523: f64, t18551: f64, t18575: f64, t18584: f64, t18593: f64, t18596: f64, t18599: f64, t18615: f64, t18630: f64, t18642: f64, t18655: f64, t15152: f64, t15153: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23053 = 16.0_f64 / 15.0_f64 * t18523;
    let t23054 = 8.0_f64 / 45.0_f64 * t18551;
    let t23055 = 8.0_f64 / 15.0_f64 * t18575;
    let t23056 = 16.0_f64 / 15.0_f64 * t18584;
    let t23057 = 16.0_f64 / 135.0_f64 * t18593;
    let t23058 = 8.0_f64 / 135.0_f64 * t18596;
    let t23059 = 8.0_f64 / 81.0_f64 * t18599;
    let t23060 = 16.0_f64 / 15.0_f64 * t18615;
    let t23061 = 16.0_f64 / 15.0_f64 * t18630;
    let t23062 = 8.0_f64 / 15.0_f64 * t18642;
    let t23064 = 4.0_f64 / 45.0_f64 * t18655;
    let t23065 = -t23053 - t23054 + t23055 - t23056 + t23057 - t23058 - t23059 - t23060 - t23061 + t23062 + t15152 - 8.0_f64 / 135.0_f64 * t15153 - t23064;
    (t23053, t23054, t23055, t23056, t23057, t23058, t23059, t23060, t23061, t23062, t23064, t23065)
}
