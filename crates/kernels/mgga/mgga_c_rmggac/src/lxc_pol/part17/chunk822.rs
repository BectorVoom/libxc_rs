//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 822/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk822<F: Float>(t2004: F, t9090: F, t2007: F, t1987: F, t1990: F, t1173: F, t674: F, t9085: F, t2868: F, t7779: F, t2186: F, t8597: F) -> (F, F, F, F, F, F, F) {
    let t40349 = t9090 * t2004;
    let t40350 = F::new(0.19863479950205658386e-4) * t40349;
    let t40351 = t9090 * t2007;
    let t40354 = t9090 * t1987;
    let t40356 = t9090 * t1990;
    let t40357 = F::new(0.19863479950205658386e-4) * t40356;
    let t40359 = t9085 * t1173 * t674;
    let t40458 = t2868 * t7779;
    let t40459 = F::new(0.79828278012425390426e-1) * t40458;
    let t40479 = t2186 * t8597;
    (t40350, t40351, t40354, t40357, t40359, t40459, t40479)
}
