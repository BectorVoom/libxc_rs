//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 948/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk948<F: Float>(t47243: F, t7427: F, t7573: F, t43497: F, t43500: F, t43502: F, t43511: F, t43514: F, t43516: F, t43519: F, t43523: F, t43527: F, t43529: F, t43567: F, t43569: F, t43571: F, t43575: F, t43579: F, t43582: F, t43588: F, t43592: F, t43597: F, t43601: F, t43602: F, t43603: F, t43604: F) -> (F, F) {
    let t47245 = t7427 * t7573 * t47243;
    let t47247 = -t43497 + t43500 + 0.14896037479937677779e-1 * t43502 - t43511 + t43514 + 0.43710935587469654631e2 * t43516 + 0.29792074959875355558e-1 * t43519 + t43523 + t43527 - 0.14896037479937677779e-1 * t43529 - 0.62115540045351614476e2 * t47245 + t43567;
    let t47249 = t43569 + t43571 - t43575 + t43579 - t43582 - 0.71500979903700853338e0 * t43588 + t43592 - t43597 + t43601 + t43602 - t43603 - t43604;
    (t47247, t47249)
}
