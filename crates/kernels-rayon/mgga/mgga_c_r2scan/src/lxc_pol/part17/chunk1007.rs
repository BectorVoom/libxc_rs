//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1007/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1007(t1058: f64, t12463: f64, t2207: f64, t3198: f64, t3290: f64, t11744: f64, t3591: f64, t10748: f64, t3187: f64, t3115: f64, t3308: f64, t10776: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12465 = t2207 * t1058 * t12463;
    let t12468 = t3290 * t3198;
    let t12470 = t11744 * t3591;
    let t12472 = t10748 * t3187;
    let t12476 = t3308 * t3115;
    let t12477 = t10776 * t12476;
    (t12465, t12468, t12470, t12472, t12476, t12477)
}
