//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 807/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk807(t8791: f64, t9033: f64, t159: f64, t619: f64, t8993: f64, t2341: f64, t309: f64, t2147: f64, t2131: f64, t1659: f64, t2127: f64, t2149: f64, t2159: f64, t2338: f64, t2342: f64, t557: f64, t616: f64, t7912: f64, t7929: f64, t7931: f64, t7944: f64, t7950: f64, t7957: f64, t8001: f64, t8400: f64, t9003: f64, t9026: f64, t9031: f64) -> (f64, f64, f64, f64, f64) {
    let t9034 = t9033 * t8791;
    let t9044 = t619 * t159 * t8993;
    let t9053 = t2341 * t309;
    let t9054 = t2147 * t9053;
    let t9055 = t2131 * t9054;
    let t9057 = -0.8673628188205199462e0_f64 * t7931 * t9026 + 0.8673628188205199462e0_f64 * t9031 - t7929 - 0.8673628188205199462e0_f64 * t8400 * t9034 + 0.8673628188205199462e0_f64 * t9003 * t2149 + 0.8673628188205199462e0_f64 * t7912 * t2342 - 0.8673628188205199462e0_f64 * t7944 + t7950 + 0.65854491829355115987e0_f64 * t7957 - 0.4336814094102599731e0_f64 * t616 * t9044 - 0.65854491829355115987e0_f64 * t2127 * t1659 - 0.4336814094102599731e0_f64 * t2338 * t2159 - 0.65854491829355115987e0_f64 * t8001 * t557 + 0.17347256376410398924e1_f64 * t9055;
    (t9034, t9044, t9054, t9055, t9057)
}
