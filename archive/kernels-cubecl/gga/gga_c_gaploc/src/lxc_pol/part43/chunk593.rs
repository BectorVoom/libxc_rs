//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 593/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk593<F: Float>(t10455: F, t1572: F, t3384: F, t4950: F, t10140: F, t1457: F, t3395: F, t6985: F, t2487: F, t10241: F, t1339: F, t590: F) -> (F, F, F, F, F, F) {
    let t10457 = F::cast_from(0.47667319935800568892e0_f64) * t1572 * t10455;
    let t10459 = F::cast_from(0.71500979903700853338e0_f64) * t4950 * t3384;
    let t10463 = t1457 * t10140;
    let t10465 = F::cast_from(0.71500979903700853338e0_f64) * t1572 * t10463;
    let t10466 = t6985 * t3395;
    let t10467 = t2487 * t10466;
    let t10468 = F::cast_from(0.25561950635947166451e0_f64) * t10467;
    let t10469 = t1339 * t10241;
    let t10470 = t10469 * t590;
    (t10457, t10459, t10465, t10468, t10469, t10470)
}
