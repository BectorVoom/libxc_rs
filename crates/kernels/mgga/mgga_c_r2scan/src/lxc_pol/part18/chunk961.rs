//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 961/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk961<F: Float>(t11563: F, t874: F, t3446: F, t3447: F, t122: F, t3434: F, t3437: F, t1103: F, t2461: F, t1053: F, t1102: F, t10653: F, t10657: F, t11478: F, t11482: F, t11485: F, t11489: F, t11492: F, t11495: F, t11500: F, t11504: F, t11508: F, t11512: F) -> (F, F, F, F, F, F) {
    let t11564 = t11563 * t874;
    let t11566 = t3446 * t3447 * t11564;
    let t11568 = t11563 * t122;
    let t11570 = t3434 * t3437 * t11568;
    let t11572 = t1103 * t2461;
    let t11574 = t1102 * t1053 * t11572;
    let t11577 = -F::new(0.15243824895787514157e-3) * t11566 + F::new(0.21684485328539747656e-4) * t11570 + t11478 + t11482 - t11485 + t11489 - t11492 + F::new(0.15243824895787514157e-3) * t11574 + t11495 + t11500 - t11504 - t11508 - t11512 + F::new(0.36021158228745895953e-3) * t10653 - t10657;
    (t11564, t11566, t11568, t11570, t11572, t11577)
}
