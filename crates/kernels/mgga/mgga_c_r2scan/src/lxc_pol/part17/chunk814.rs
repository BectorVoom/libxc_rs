//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 814/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk814<F: Float>(t2148: F, t9445: F, t2147: F, t1632: F, t3216: F, t551: F, t549: F, t2670: F, t2731: F, t133: F, t255: F, t3177: F, t546: F, t2236: F, t3192: F, t3218: F, t562: F, t6218: F, t6463: F, t6465: F, t6478: F, t6483: F, t8201: F, t8224: F, t8227: F, t8231: F, t8234: F, t8245: F, t9436: F, t9441: F) -> (F, F, F) {
    let t9446 = t2148 * t9445;
    let t9447 = t2147 * t9446;
    let t9451 = t1632 * t3216;
    let t9452 = t551 * t9451;
    let t9453 = t549 * t9452;
    let t9458 = t2670 * t2731;
    let t9463 = t133 * t3177 * t255;
    let t9464 = t546 * t9463;
    let t9467 = -0.58218257753910989057e-2 * t9436 + 0.84755945902752848174e0 * t8201 - 0.2600466522016280569e0 * t6218 * t9441 - t8224 - 0.32927245914677557993e-1 * t8227 + t8231 - t8234 - 0.58218257753910989057e-2 * t9447 - 0.43341108700271342816e-1 * t2236 * t3218 + 0.11557628986739024751e0 * t9453 - t8245 - 0.28914548798370980346e-3 * t6463 - 0.63479958930231934629e-2 * t6478 - 0.19043987679069580389e-1 * t6483 + 0.69345773920434148507e0 * t9458 + 0.86682217400542685632e-1 * t6465 * t3192 - 0.43341108700271342816e-1 * t9464 * t562;
    (t9451, t9463, t9467)
}
