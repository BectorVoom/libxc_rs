//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 964/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk964<F: Float>(t1392: F, t1979: F, t1982: F, t201: F, t457: F, t7428: F, t8688: F, t2004: F, t9087: F, t2412: F, t7677: F, t2007: F) -> (F, F, F, F, F) {
    let t40502 = t1392 * t457 * t201 * t1979 * t1982;
    let t40505 = t8688 * t7428 * t1982;
    let t40506 = F::cast_from(0.19863479950205658386e-4_f64) * t40505;
    let t40507 = t9087 * t2004;
    let t40509 = t2412 * t7677;
    let t40511 = t9087 * t2007;
    (t40502, t40506, t40507, t40509, t40511)
}
