//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 744/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk744<F: Float>(t2323: F, t638: F, t7184: F, t2004: F, t9090: F, t2007: F, t1987: F, t1990: F, t1173: F, t674: F, t9085: F, t2868: F, t7779: F, t2186: F, t8597: F, t1982: F, t7428: F, t8688: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40343 = t638 * t7184 * t2323;
    let t40349 = t9090 * t2004;
    let t40351 = t9090 * t2007;
    let t40354 = t9090 * t1987;
    let t40356 = t9090 * t1990;
    let t40359 = t9085 * t1173 * t674;
    let t40458 = t2868 * t7779;
    let t40479 = t2186 * t8597;
    let t40505 = t8688 * t7428 * t1982;
    (t40343, t40349, t40351, t40354, t40356, t40359, t40458, t40479, t40505)
}
