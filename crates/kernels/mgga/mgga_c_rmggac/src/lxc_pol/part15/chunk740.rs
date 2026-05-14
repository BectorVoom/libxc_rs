//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 740/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk740<F: Float>(t1990: F, t9090: F, t1173: F, t674: F, t9085: F, t2868: F, t7779: F, t2186: F, t8597: F, t1982: F, t7428: F, t8688: F, t1627: F, t2064: F, t3928: F, t34884: F, t8668: F) -> (F, F, F, F, F, F, F) {
    let t40356 = t9090 * t1990;
    let t40357 = 0.19863479950205658386e-4 * t40356;
    let t40359 = t9085 * t1173 * t674;
    let t40458 = t2868 * t7779;
    let t40459 = 0.79828278012425390426e-1 * t40458;
    let t40479 = t2186 * t8597;
    let t40480 = 0.19863479950205658386e-4 * t40479;
    let t40505 = t8688 * t7428 * t1982;
    let t40506 = 0.19863479950205658386e-4 * t40505;
    let t40516 = t3928 * t2064 * t1627;
    let t40558 = t34884 * t8668;
    (t40357, t40359, t40459, t40480, t40506, t40516, t40558)
}
