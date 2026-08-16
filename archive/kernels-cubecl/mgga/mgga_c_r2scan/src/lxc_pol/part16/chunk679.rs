//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 679/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk679<F: Float>(t1398: F, t5: F, t753: F, t1767: F, t2021: F, t1762: F, t1978: F, t1818: F, t377: F, t1983: F, t1763: F, t1949: F) -> (F, F, F, F, F) {
    let t5195 = t5 * t1398 * t753;
    let t5200 = t1767 * t2021;
    let t5202 = F::cast_from(0.97592231702715658578e-1_f64) * t1762 * t5200;
    let t5203 = t1767 * t1978;
    let t5205 = F::cast_from(0.48159733137676571079e0_f64) * t1762 * t5203;
    let t5206 = t377 * t1818;
    let t5207 = t5206 * t1983;
    let t5209 = F::cast_from(0.28518989949414381017e2_f64) * t1762 * t5207;
    let t5210 = t1763 * t1949;
    (t5195, t5202, t5205, t5209, t5210)
}
