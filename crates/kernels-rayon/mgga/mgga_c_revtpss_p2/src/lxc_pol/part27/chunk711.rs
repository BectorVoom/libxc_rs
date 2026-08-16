//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 711/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk711(t1949: f64, t231: f64, t836: f64, t7076: f64, t233: f64, t7048: f64, t1957: f64, t1956: f64, t1959: f64, t213: f64, t257: f64, t7017: f64, t7020: f64, t7049: f64, t7053: f64, t7062: f64, t7066: f64, t7067: f64, t7070: f64, t7073: f64, t887: f64) -> (f64, f64, f64, f64, f64) {
    let t7078 = t1949 * t836 * t231;
    let t7079 = t7076 * t7078;
    let t7082 = t233 * t7048;
    let t7083 = t1957 * t7082;
    let t7086 = -t7017 + t7020 + 0.65854491829355115987e0_f64 * t213 * t7049 * t257 - 0.65854491829355115987e0_f64 * t7053 * t887 + t7062 - t7066 - 0.4336814094102599731e0_f64 * t7067 * t1959 + 0.8673628188205199462e0_f64 * t7070 * t7073 + 0.4336814094102599731e0_f64 * t7070 * t7079 - 0.4336814094102599731e0_f64 * t1956 * t7083;
    (t7078, t7079, t7082, t7083, t7086)
}
