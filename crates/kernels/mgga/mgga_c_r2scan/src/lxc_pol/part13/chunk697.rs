//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 697/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk697<F: Float>(t5317: F, t720: F, t748: F, t234: F, t1654: F, t1861: F, t1860: F, t170: F, t1871: F, t597: F, t1853: F, t625: F, t645: F) -> (F, F, F, F, F, F, F) {
    let t5318 = t720 * t5317;
    let t5319 = t748 * t5318;
    let t5321 = F::cast_from(0.17315859105681463759e2_f64) * t234 * t5319;
    let t5322 = t1654 * t1861;
    let t5323 = t1860 * t5322;
    let t5325 = t170 * t1871;
    let t5326 = t597 * t5325;
    let t5327 = t1860 * t5326;
    let t5331 = F::cast_from(0.71233333333333333332e-1_f64) * t625 * t1853 * t645;
    (t5321, t5322, t5323, t5325, t5326, t5327, t5331)
}
