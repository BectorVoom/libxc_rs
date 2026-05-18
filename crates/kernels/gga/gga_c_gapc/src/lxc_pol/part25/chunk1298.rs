//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 1298/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk1298<F: Float>(t11216: F, t1448: F, t4055: F, t11204: F, t25127: F, t11211: F, t25117: F, t11227: F, t8286: F, t8291: F, t11202: F, t128: F, t15354: F, t25054: F) -> (F, F, F, F, F) {
    let t35478 = t11216 * t1448 * t4055;
    let t35480 = t11204 * t25127;
    let t35482 = t25117 * t11211;
    let t35485 = t8286 * t11227 * t8291;
    let t35489 = t11202 * t15354 * t128 * t25054;
    (t35478, t35480, t35482, t35485, t35489)
}
