//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 253/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk253(t1156: f64, t197: f64, t1144: f64, t1149: f64, t1152: f64, t198: f64, t446: f64, t454: f64, t998: f64, t201: f64, t457: f64, t461: f64, t495: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1157 = t197 * t1156;
    let t1162 = -0.32163648644302209643e2_f64 * t1149 * t198 + 0.19298189186581325786e3_f64 * t1152 * t446 - 0.38596378373162651572e3_f64 * t1157 * t1144 + 0.96490945932906628929e2_f64 * t454 * t998;
    let t1163 = t1162 * t201;
    let t1165 = t457 * t457;
    let t1166 = t1165 * t201;
    let t1168 = t461 * t495;
    (t1157, t1162, t1163, t1165, t1166, t1168)
}
