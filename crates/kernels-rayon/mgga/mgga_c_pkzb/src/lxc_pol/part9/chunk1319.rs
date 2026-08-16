//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1319/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1319(t154: f64, t3026: f64, t385: f64, t6446: f64, t1220: f64, t6452: f64, t2347: f64, t7945: f64, t6448: f64, t10258: f64, t19133: f64, t19153: f64, t19158: f64, t19163: f64, t2185: f64, t2226: f64, t22260: f64, t23278: f64, t23286: f64, t23296: f64, t23299: f64, t23311: f64, t23313: f64, t2411: f64, t3174: f64, t3235: f64, t6387: f64, t6404: f64, t6443: f64, t758: f64, t7984: f64, t824: f64, t8409: f64, t907: f64) -> f64 {
    let t23317 = t385 * t154 * t6446 * t3026;
    let t23318 = t23317 / 144.0_f64;
    let t23319 = t1220 * t6452;
    let t23325 = t385 * t154 * t2347 * t7945;
    let t23331 = t1220 * t6448;
    let t23332 = t23331 / 54.0_f64;
    let t23333 = -0.28582678745379824648e-3_f64 * t19133 - 3.0_f64 / 16.0_f64 * t3174 * t23278 * t7984 * t824 - 0.3811023832717309953e-3_f64 * t19153 - 0.85748036236139473943e-3_f64 * t19158 - t19163 - 0.10289764348336736874e-1_f64 * t23286 - 0.15434646522505105311e-1_f64 * t3235 * t758 * t6404 * t3026 * t2226 - 0.20579528696673473746e-1_f64 * t10258 * t6387 + 0.51448821741683684367e-2_f64 * t23296 + 0.25724410870841842184e-2_f64 * t23299 + 0.38586616306262763276e-2_f64 * t3235 * t758 * t2411 * t7945 * t824 + 0.38586616306262763276e-2_f64 * t3235 * t758 * t8409 * t2185 + 0.17149607247227894789e-2_f64 * t23311 - t23313 / 18.0_f64 + t23318 + t23319 / 36.0_f64 + t1220 * t6443 / 36.0_f64 - t23325 / 96.0_f64 - t385 * t154 * t907 * t22260 / 96.0_f64 - t23332;
    t23333
}
