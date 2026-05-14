//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 823/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk823<F: Float>(t806: F, t9597: F, t35: F, t990: F, t1216: F, t1248: F, t2904: F, t4911: F, t298: F, t2916: F, t6635: F, t810: F, t1000: F, t1256: F, t2920: F, t308: F) -> (F, F, F, F, F, F, F, F) {
    let t9598 = t9597 * t806;
    let t9601 = t990 * t35;
    let t9602 = t9601 * t1216;
    let t9607 = t1248 * t2904;
    let t9608 = t9607 * t806;
    let t9612 = -t1216 - 3.0 * t4911;
    let t9613 = t298 * t9612;
    let t9622 = t6635 * t2916;
    let t9623 = t9622 * t810;
    let t9626 = t1000 * t35;
    let t9627 = t9626 * t1216;
    let t9630 = t1256 * t2920;
    let t9631 = t9630 * t810;
    let t9634 = -t9612;
    let t9635 = t308 * t9634;
    (t9598, t9602, t9608, t9613, t9623, t9627, t9631, t9635)
}
