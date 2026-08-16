//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 554/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk554(t3181: f64, t66: f64, t2853: f64, t247: f64, t1041: f64, t1047: f64, t1063: f64, t1068: f64, t3082: f64, t3083: f64, t3086: f64, t3091: f64, t3097: f64, t3101: f64, t3106: f64, t3112: f64, t3115: f64, t3120: f64, t3124: f64, t3127: f64, t3130: f64, t3136: f64, t3150: f64, t3157: f64, t3161: f64, t3164: f64, t3169: f64, t3174: f64, t3177: f64, t348: f64) -> (f64, f64, f64) {
    let t3182 = t66 * t3181;
    let t3183 = t3182 * t2853;
    let t3184 = t247 * t3183;
    let t3187 = -t3082 + 11.0_f64 / 108.0_f64 * t3083 * t348 - t3086 / 54.0_f64 + 0.28582678745379824648e-3_f64 * t3091 * t3097 - 0.28582678745379824648e-3_f64 * t1063 * t3101 - 0.15244095330869239812e-2_f64 * t3106 * t1068 + 0.19055119163586549765e-3_f64 * t3112 - 0.42874018118069736972e-3_f64 * t3115 * t3120 + 0.42874018118069736972e-3_f64 * t3124 * t1047 - 0.28582678745379824648e-3_f64 * t3127 * t3130 + 0.21437009059034868486e-3_f64 * t1041 * t3136 + 0.42874018118069736972e-3_f64 * t3150 * t3157 - 0.21437009059034868486e-3_f64 * t3161 * t3164 - 0.22866142996303859718e-2_f64 * t3169 * t1047 + 0.28582678745379824648e-3_f64 * t3174 + 0.14291339372689912324e-3_f64 * t1063 * t3177 + 0.23818898954483187207e-3_f64 * t1063 * t3184;
    (t3182, t3184, t3187)
}
