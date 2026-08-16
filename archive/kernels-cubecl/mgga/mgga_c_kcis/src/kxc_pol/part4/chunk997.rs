//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 997/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk997<F: Float>(t1280: F, t433: F, t1409: F, t1471: F, t1317: F, t1392: F, t544: F, t3751: F, t456: F, t3752: F, t518: F, t3255: F, t3763: F) -> (F, F, F, F, F, F) {
    let t11228 = t1280 * t1280;
    let t11229 = F::cast_from(1.0_f64) / t11228;
    let t11230 = t433 * t11229;
    let t11322 = t1471 * t1409;
    let t11332 = t1392 * t1317 * t544;
    let t11369 = t3751 * t456 * t544;
    let t11374 = t3752 * t518;
    let t11379 = t3255 * t3763;
    (t11230, t11322, t11332, t11369, t11374, t11379)
}
