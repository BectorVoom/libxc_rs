//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 794/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk794<F: Float>(t2572: F, t4359: F, t995: F, t4372: F, t986: F, t2594: F, t2597: F, t1003: F, t1436: F, t260: F, t3608: F, t4289: F, t4291: F, t4295: F, t4321: F, t4324: F, t4355: F, t4379: F) -> (F, F, F, F, F) {
    let t4386 = t2572 * t4359 * t995;
    let t4390 = t986 * t4372 * t995;
    let t4393 = t2594 * t4359;
    let t4394 = t4393 * t2597;
    let t4397 = -t4289 + t4291 - t4295 + t4321 + t4324 + t260 * t4379 + 0.19751673498613801407e-1 * t260 * t4355 - 0.11696447245269292414e1 * t3608 * t1436 + 0.11696447245269292414e1 * t1003 * t4386 - 0.5848223622634646207e0 * t1003 * t4390 - 0.17315859105681463759e2 * t1003 * t4394;
    (t4386, t4390, t4393, t4394, t4397)
}
