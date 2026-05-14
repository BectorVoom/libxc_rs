//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 966/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk966<F: Float>(t8322: F, t8371: F, t8405: F, t8478: F, t158: F, t1255: F, t2429: F, t6546: F, t2428: F, t3278: F, t951: F, t2453: F, t3254: F, t3258: F, t6514: F, t7832: F, t8430: F) -> (F, F, F, F, F, F, F, F) {
    let t8480 = t8322 + t8371 + t8405 + t8478;
    let t8481 = t8480 * t158;
    let t8497 = t6546 * t1255 * t2429;
    let t8500 = t2428 * t3278;
    let t8501 = t8500 * t951;
    let t8504 = t3254 * t2453;
    let t8507 = t6514 * t3258;
    let t8508 = t7832 * t8430;
    (t8480, t8481, t8497, t8500, t8501, t8504, t8507, t8508)
}
