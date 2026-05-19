//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 649/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk649<F: Float>(t413: F, t3609: F, t1260: F, t286: F, t1251: F, t1255: F, t1264: F, t3484: F, t3487: F, t3490: F, t3499: F, t3502: F, t3505: F, t3510: F, t3514: F, t3517: F, t3522: F, t3526: F, t3534: F, t423: F) -> (F, F, F) {
    let t418 = F::new(0.0) < t413;
    let t3611 = piecewise3::<F>(t418, t3609, -t3609);
    let t3612 = t1260 * t3611;
    let t3613 = t286 * t3612;
    let t3616 = F::new(11.0) / F::new(216.0) * t3484 * t423 - t3487 / F::new(108.0) - t3490 * t1255 / F::new(108.0) + t3490 * t1264 / F::new(36.0) - t3499 + t3502 / F::new(864.0) - t3505 / F::new(288.0) + t1251 * t3510 / F::new(432.0) - t3514 * t3517 / F::new(288.0) - t1251 * t3522 / F::new(288.0) + t1251 * t3526 / F::new(576.0) + t1251 * t3534 / F::new(96.0) - t1251 * t3613 / F::new(192.0);
    (t3611, t3612, t3616)
}
