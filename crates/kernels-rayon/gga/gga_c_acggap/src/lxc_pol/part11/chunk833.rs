//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 833/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk833(t1603: f64, t615: f64, t2331: f64, t315: f64, t323: f64, t557: f64, t7973: f64, t2341: f64, t322: f64, t2147: f64, t2138: f64, t2347: f64, t621: f64, t7912: f64, t7962: f64, t7967: f64, t7974: f64, t7977: f64, t7981: f64, t7985: f64, t7988: f64, t7991: f64, t7996: f64, t8000: f64) -> (f64, f64, f64, f64) {
    let t9058 = t615 * t1603;
    let t9062 = t315 * t2331;
    let t9063 = t9062 * t323;
    let t9073 = t7973 * t557;
    let t9075 = t2341 * t322;
    let t9076 = t2147 * t9075;
    let t9077 = t2138 * t9076;
    let t9079 = t7962 - 0.4336814094102599731e0_f64 * t9058 * t621 + 0.8673628188205199462e0_f64 * t7967 - 0.65854491829355115987e0_f64 * t9063 - 0.65854491829355115987e0_f64 * t7974 - 0.65854491829355115987e0_f64 * t7977 + 0.4336814094102599731e0_f64 * t7912 * t2347 - 0.8673628188205199462e0_f64 * t7981 + 0.8673628188205199462e0_f64 * t7985 - 0.8673628188205199462e0_f64 * t7988 + 0.8673628188205199462e0_f64 * t7991 + t7996 - t8000 - 0.65854491829355115987e0_f64 * t9073 - 0.17347256376410398924e1_f64 * t9077;
    (t9058, t9062, t9076, t9079)
}
