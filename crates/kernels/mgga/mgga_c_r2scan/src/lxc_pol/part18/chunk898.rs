//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 898/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk898<F: Float>(t298: F, t9612: F, t2916: F, t6635: F, t810: F, t1000: F, t35: F, t1216: F, t1256: F, t2920: F, t308: F, t2369: F, t2373: F, t2901: F, t2905: F, t2911: F, t295: F, t305: F, t6648: F, t803: F, t811: F, t8319: F, t8340: F, t9598: F, t9602: F, t9608: F, t997: F) -> (F, F, F, F, F) {
    let t9613 = t298 * t9612;
    let t9622 = t6635 * t2916;
    let t9623 = t9622 * t810;
    let t9626 = t1000 * t35;
    let t9627 = t9626 * t1216;
    let t9630 = t1256 * t2920;
    let t9631 = t9630 * t810;
    let t9634 = -t9612;
    let t9635 = t308 * t9634;
    let t9638 = -F::new(50.0) / F::new(27.0) * t803 * t2901 - F::new(10.0) / F::new(27.0) * t295 * t9598 + F::new(20.0) / F::new(9.0) * t8319 * t9602 - F::new(25.0) / F::new(9.0) * t803 * t2905 + F::new(10.0) / F::new(9.0) * t295 * t9608 + F::new(5.0) / F::new(3.0) * t295 * t9613 + F::new(200.0) / F::new(27.0) * t2911 * t811 - F::new(100.0) / F::new(27.0) * t997 * t2369 + F::new(50.0) / F::new(9.0) * t997 * t2373 - F::new(10.0) / F::new(27.0) * t305 * t9623 - F::new(20.0) / F::new(9.0) * t8340 * t9627 + F::new(10.0) / F::new(9.0) * t305 * t9631 + F::new(5.0) / F::new(3.0) * t305 * t9635 + t6648;
    (t9613, t9623, t9631, t9635, t9638)
}
