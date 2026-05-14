//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 829/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk829<F: Float>(t2191: F, t9935: F, t1986: F, t6592: F, t675: F, t1743: F, t352: F, t1756: F, t7567: F, t2160: F, t638: F, t9754: F, t9750: F, t9746: F, t1525: F, t236: F, t618: F, t7230: F, t7231: F) -> (F, F, F, F, F, F, F, F) {
    let t45614 = t2191 * t9935;
    let t45617 = t675 * t1986 * t6592;
    let t45622 = t1743 * t352;
    let t45626 = t7567 * t1756;
    let t45630 = t638 * t2160 * t9754;
    let t45633 = t638 * t2160 * t9750;
    let t45636 = t638 * t2160 * t9746;
    let t45641 = t7230 * t7231 * t236 * t618 * t1525;
    (t45614, t45617, t45622, t45626, t45630, t45633, t45636, t45641)
}
