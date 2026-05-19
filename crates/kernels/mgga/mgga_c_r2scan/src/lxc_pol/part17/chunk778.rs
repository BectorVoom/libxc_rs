//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 778/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk778<F: Float>(t1592: F, t7555: F, t2832: F, t537: F, t255: F, t571: F, t1600: F, t2631: F, t551: F, t6343: F, t921: F, t574: F) -> (F, F, F, F) {
    let t7557 = F::cast_from(0.69345773920434148506e0_f64) * t1592 * t7555;
    let t7564 = t537 * t2832;
    let t7566 = t571 * t7564 * t255;
    let t7582 = F::cast_from(0.12805040077930161442e0_f64) * t1600 * t2631;
    let t7597 = t551 * t6343 * t921;
    let t7598 = t574 * t7597;
    (t7557, t7566, t7582, t7598)
}
