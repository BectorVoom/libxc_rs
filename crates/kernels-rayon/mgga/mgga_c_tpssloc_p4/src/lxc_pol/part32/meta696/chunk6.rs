//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2167/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2167(t1339: f64, t22827: f64, t550: f64, t74366: f64, t1307: f64, t6415: f64, t6420: f64, t1825: f64, t5286: f64, t6936: f64, t57091: f64, t91144: f64, t91155: f64, t91159: f64, t91162: f64, t91171: f64, t91180: f64, t93650: f64, t93656: f64, t97273: f64, t97277: f64, t97281: f64, t97283: f64, t97287: f64) -> f64 {
    let t97291 = t22827 * t1339 * t74366 * t550;
    let t97295 = t22827 * t1339 * t6415 * t1307;
    let t97299 = t22827 * t1339 * t6420 * t1307;
    let t97303 = t6936 * t1339 * t1825 * t5286;
    let t97307 = t6936 * t1339 * t57091 * t550;
    let t97309 = -t91144 + 0.24223653656484234512e-2_f64 * t97273 + 0.24223653656484234512e-2_f64 * t97277 - 0.24223653656484234512e-2_f64 * t97281 - t93650 + t91155 - t91159 + t91162 - 35.0_f64 / 576.0_f64 * t97283 - 0.84782787797694820792e-2_f64 * t97287 + 0.12111826828242117256e-2_f64 * t97291 + 0.12111826828242117256e-2_f64 * t97295 + 0.12111826828242117256e-2_f64 * t97299 - 0.40372756094140390854e-3_f64 * t97303 - 0.20186378047070195427e-3_f64 * t97307 - t93656 - t91171 - t91180;
    t97309
}
