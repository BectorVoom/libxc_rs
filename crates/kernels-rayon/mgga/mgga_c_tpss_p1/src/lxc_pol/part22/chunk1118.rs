//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1118/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1118(t1072: f64, t1081: f64, t12135: f64, t1089: f64, t12159: f64, t12161: f64, t12164: f64, t12167: f64, t12170: f64, t12243: f64, t12246: f64, t12250: f64, t12253: f64, t12257: f64, t12260: f64, t12276: f64, t12337: f64, t12340: f64, t12342: f64, t12344: f64, t12346: f64) -> (f64, f64) {
    let t12348 = t1072 * t12135 * t1081;
    let t12350 = 0.5848223622634646207e0_f64 * t1089 * t12348;
    let t12351 = t12276 - t12159 + t12161 - t12164 - t12167 - t12170 - t12243 - t12246 + t12250 + t12253 + t12257 + t12260 + t12337 + t12340 - t12342 - t12344 - t12346 - t12350;
    (t12350, t12351)
}
