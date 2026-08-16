//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1327/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1327(t11153: f64, t931: f64, t11445: f64, t154: f64, t18994: f64, t385: f64, t2347: f64, t3171: f64, t3849: f64, t10038: f64, t11383: f64, t1220: f64, t19055: f64, t19124: f64, t19153: f64, t19163: f64, t23272: f64, t2888: f64, t31086: f64, t3174: f64, t3181: f64, t824: f64, t907: f64, t909: f64) -> f64 {
    let t32143 = t931 * t11153;
    let t32150 = t385 * t154 * t18994 * t11445;
    let t32164 = t385 * t154 * t2347 * t11153;
    let t32166 = t3849 * t3171;
    let t32168 = -t19055 + 0.63517063878621832551e-4_f64 * t19124 - 0.38110238327173099531e-3_f64 * t23272 - 0.1270341277572436651e-3_f64 * t19153 - t19163 + t3174 * t2888 * t32143 * t824 / 48.0_f64 - t32150 / 48.0_f64 - t385 * t154 * t907 * t31086 / 96.0_f64 + 77.0_f64 / 162.0_f64 * t11383 * t909 - 11.0_f64 / 36.0_f64 * t3849 * t3181 + t1220 * t10038 / 12.0_f64 - t32164 / 288.0_f64 - 11.0_f64 / 108.0_f64 * t32166;
    t32168
}
