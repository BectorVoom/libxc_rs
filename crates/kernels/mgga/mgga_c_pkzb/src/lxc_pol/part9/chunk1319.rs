//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1319/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1319<F: Float>(t154: F, t3026: F, t385: F, t6446: F, t1220: F, t6452: F, t2347: F, t7945: F, t6448: F, t10258: F, t19133: F, t19153: F, t19158: F, t19163: F, t2185: F, t2226: F, t22260: F, t23278: F, t23286: F, t23296: F, t23299: F, t23311: F, t23313: F, t2411: F, t3174: F, t3235: F, t6387: F, t6404: F, t6443: F, t758: F, t7984: F, t824: F, t8409: F, t907: F) -> F {
    let t23317 = t385 * t154 * t6446 * t3026;
    let t23318 = t23317 / F::new(144.0);
    let t23319 = t1220 * t6452;
    let t23325 = t385 * t154 * t2347 * t7945;
    let t23331 = t1220 * t6448;
    let t23332 = t23331 / F::new(54.0);
    let t23333 = -F::cast_from(0.28582678745379824648e-3_f64) * t19133 - F::new(3.0) / F::new(16.0) * t3174 * t23278 * t7984 * t824 - F::cast_from(0.3811023832717309953e-3_f64) * t19153 - F::cast_from(0.85748036236139473943e-3_f64) * t19158 - t19163 - F::cast_from(0.10289764348336736874e-1_f64) * t23286 - F::cast_from(0.15434646522505105311e-1_f64) * t3235 * t758 * t6404 * t3026 * t2226 - F::cast_from(0.20579528696673473746e-1_f64) * t10258 * t6387 + F::cast_from(0.51448821741683684367e-2_f64) * t23296 + F::cast_from(0.25724410870841842184e-2_f64) * t23299 + F::cast_from(0.38586616306262763276e-2_f64) * t3235 * t758 * t2411 * t7945 * t824 + F::cast_from(0.38586616306262763276e-2_f64) * t3235 * t758 * t8409 * t2185 + F::cast_from(0.17149607247227894789e-2_f64) * t23311 - t23313 / F::new(18.0) + t23318 + t23319 / F::new(36.0) + t1220 * t6443 / F::new(36.0) - t23325 / F::new(96.0) - t385 * t154 * t907 * t22260 / F::new(96.0) - t23332;
    t23333
}
