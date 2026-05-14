//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1061/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1061<F: Float>(t5520: F, t5522: F, t7357: F, t7359: F, t9148: F, t9163: F, t665: F, t3528: F, t5547: F, t667: F, t2759: F, t2765: F, t5543: F, t7451: F, t9138: F, t9140: F, t9143: F) -> (F, F, F, F, F, F) {
    let t9164 = -t5520 + 4.0 / 9.0 * t5522 + 8.0 / 9.0 * t7357 - t7359 - t9148 / 3.0 + t9163;
    let t9165 = t665 * t9164;
    let t9171 = t5547 * t3528;
    let t9172 = t9171 * t667;
    let t9174 = t2765 * t2759;
    let t9176 = 0.19419375e1 * t9138 - 0.258925e1 * t9140 - 0.1294625e1 * t9143 + 0.258925e1 * t9165 - t5543 + 0.40256666666666666667e0 * t5522 + 0.80513333333333333333e0 * t7357 - t7451 - 0.301925e0 * t9148 + 0.905775e0 * t9163 - 0.412621875e-1 * t9172 + 0.16504875e0 * t9174;
    (t9164, t9165, t9171, t9172, t9174, t9176)
}
