//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2310/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2310(t8070: f64, t94490: f64, t86036: f64, t95760: f64, t103779: f64, t1409: f64, t1734: f64, t19138: f64, t24849: f64, t24851: f64, t27502: f64, t27507: f64, t27532: f64, t27540: f64, t29735: f64, t3624: f64, t3966: f64, t5011: f64, t6256: f64, t7327: f64, t7376: f64, t8082: f64, t86015: f64, t86116: f64, t95098: f64, t95114: f64, t95197: f64, t95201: f64, t95761: f64) -> f64 {
    let t103830 = t94490 * t8070;
    let t103838 = t86036 * t95760;
    let t103864 = 0.14621636149762012769e-1_f64 * t103830 - 0.54831135561607547883e-2_f64 * t24849 * t7327 * t6256 * t27532 - 0.16449340668482264365e-1_f64 * t95761 * t27540 - 0.3289868133696452873e-1_f64 * t103838 * t95197 + 0.16449340668482264365e-1_f64 * t103838 * t95201 - 0.54831135561607547884e-2_f64 * t24849 * t86116 * t29735 - 0.54831135561607547884e-2_f64 * t24849 * t24851 * t3966 * t1734 * t7376 - t95098 - t95114 - 0.43864908449286038306e-1_f64 * t27507 * t27502 - 0.54831135561607547884e-2_f64 * t24849 * t24851 * t1409 * t5011 * t7376 - 0.54831135561607547884e-2_f64 * t24849 * t86015 * t103779 - 2.0_f64 * t3624 * t8082 * t19138;
    t103864
}
