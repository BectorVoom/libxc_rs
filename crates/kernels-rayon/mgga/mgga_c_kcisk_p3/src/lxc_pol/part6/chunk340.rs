//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 340/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk340(t1173: f64, t2083: f64, t1180: f64, t1186: f64, t2075: f64, t26: f64, t1178: f64, t1185: f64, t2077: f64, t1191: f64, t1172: f64, t1195: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2084 = t1173 * t2083;
    let t2087 = t1180 * t2083;
    let t2089 = t1186 * t2075;
    let t2090 = t26 * t2089;
    let t2092 = 0.1898925e1_f64 * t2084 - t1178 - 0.29896666666666666667e0_f64 * t2077 + 0.3071625e0_f64 * t2087 - t1185 - 0.82156666666666666667e-1_f64 * t2090;
    let t2093 = t2092 * t1191;
    let t2095 = 1.0_f64 * t1172 * t2093;
    let t2097 = -t1195 - 0.92708333333333333333e-2_f64 * t2077;
    (t2084, t2087, t2089, t2090, t2092, t2093, t2095, t2097)
}
