//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 618/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk618<F: Float>(t1251: F, t1255: F, t1264: F, t3484: F, t3487: F, t3490: F, t3499: F, t3502: F, t3505: F, t3510: F, t3514: F, t3517: F, t3522: F, t3526: F, t3534: F, t3613: F, t423: F) -> (F,) {
    let t3616 = 11.0 / 216.0 * t3484 * t423 - t3487 / 108.0 - t3490 * t1255 / 108.0 + t3490 * t1264 / 36.0 - t3499 + t3502 / 864.0 - t3505 / 288.0 + t1251 * t3510 / 432.0 - t3514 * t3517 / 288.0 - t1251 * t3522 / 288.0 + t1251 * t3526 / 576.0 + t1251 * t3534 / 96.0 - t1251 * t3613 / 192.0;
    (t3616,)
}
