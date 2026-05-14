//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 793/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk793<F: Float>(t41689: F, t34264: F, t7030: F, t10177: F, t10523: F, t544: F, t899: F, t913: F, t12957: F, t1441: F, t39968: F, t10122: F, t1445: F, t2293: F, t574: F, t12939: F, t1407: F) -> (F, F, F, F, F, F, F) {
    let t41690 = 0.17041300423964777634e0 * t41689;
    let t41691 = t34264 * t7030;
    let t41692 = 0.29792074959875355558e-1 * t41691;
    let t41696 = t544 * t10523 * t899 * t913 * t10177;
    let t41697 = 0.17875244975925213335e0 * t41696;
    let t41698 = t1441 * t12957;
    let t41699 = 0.1022478025437886658e1 * t41698;
    let t41700 = 0.19171462976960374838e1 * t39968;
    let t41703 = t574 * t1445 * t10122 * t2293;
    let t41705 = t1407 * t12939;
    (t41690, t41692, t41697, t41699, t41700, t41703, t41705)
}
