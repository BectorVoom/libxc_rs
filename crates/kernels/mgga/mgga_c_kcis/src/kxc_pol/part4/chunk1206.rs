//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1206/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1206<F: Float>(t413: F, t15291: F, t15468: F, t1260: F, t286: F, t110: F, t1852: F, t1251: F, t15255: F, t4580: F, t3515: F, t330: F, t421: F, t992: F) -> (F, F, F, F, F) {
    let t418 = F::new(0.0) < t413;
    let t15469 = t15291 + t15468;
    let t15471 = piecewise3::<F>(t418, t15469, -t15469);
    let t15472 = t1260 * t15471;
    let t15473 = t286 * t15472;
    let t15476 = t110 * t1852;
    let t15477 = t1251 * t15476;
    let t15481 = t4580 * t15255;
    let t15482 = t3515 * t15481;
    let t15486 = t992 * t421 * t330;
    (t15469, t15473, t15477, t15482, t15486)
}
