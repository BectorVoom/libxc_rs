//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 886/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk886<F: Float>(t546: F, t9463: F, t2236: F, t3192: F, t3218: F, t562: F, t6218: F, t6463: F, t6465: F, t6478: F, t6483: F, t8201: F, t8224: F, t8227: F, t8231: F, t8234: F, t8245: F, t9436: F, t9441: F, t9447: F, t9453: F, t9458: F) -> F {
    let t9464 = t546 * t9463;
    let t9467 = -F::cast_from(0.58218257753910989057e-2_f64) * t9436 + F::cast_from(0.84755945902752848174e0_f64) * t8201 - F::cast_from(0.2600466522016280569e0_f64) * t6218 * t9441 - t8224 - F::cast_from(0.32927245914677557993e-1_f64) * t8227 + t8231 - t8234 - F::cast_from(0.58218257753910989057e-2_f64) * t9447 - F::cast_from(0.43341108700271342816e-1_f64) * t2236 * t3218 + F::cast_from(0.11557628986739024751e0_f64) * t9453 - t8245 - F::cast_from(0.28914548798370980346e-3_f64) * t6463 - F::cast_from(0.63479958930231934629e-2_f64) * t6478 - F::cast_from(0.19043987679069580389e-1_f64) * t6483 + F::cast_from(0.69345773920434148507e0_f64) * t9458 + F::cast_from(0.86682217400542685632e-1_f64) * t6465 * t3192 - F::cast_from(0.43341108700271342816e-1_f64) * t9464 * t562;
    t9467
}
