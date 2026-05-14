//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1220/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1220<F: Float>(t10615: F, t31167: F, t6703: F, t8248: F, t10578: F, t10590: F, t1508: F, t1599: F, t193: F, t30712: F, t30735: F, t31623: F, t3371: F, t3387: F, t3402: F, t34498: F, t34500: F, t34503: F, t34510: F, t34512: F, t4598: F, t4631: F, t524: F, t568: F, t569: F, t574: F) -> (F,) {
    let t34530 = t10615 * t31167;
    let t34531 = 0.44688112439813033337e-1 * t34530;
    let t34533 = 0.2780593662921699852e0 * t8248 * t6703;
    let t34534 = -t34498 - t34500 - t34503 + t34510 + t34512 - t30712 - 0.23005755572352449806e1 * t574 * t568 * t569 * t31623 + 0.35750489951850426669e0 * t1508 * t3371 * t193 + 0.71500979903700853338e0 * t524 * t10590 * t193 - 0.35750489951850426669e0 * t4631 * t3387 - 0.71500979903700853338e0 * t1599 * t10578 - 0.1022478025437886658e1 * t574 * t4598 * t3402 - t34531 - t30735 - t34533;
    (t34534,)
}
