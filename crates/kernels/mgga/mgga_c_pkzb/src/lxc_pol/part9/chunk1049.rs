//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1049/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1049<F: Float>(t1497: F, t1504: F, t4911: F, t4915: F, t555: F, t1528: F, t204: F, t5063: F, t148: F, t1598: F, t1602: F, t1527: F, t5008: F) -> (F, F, F, F) {
    let t16280 = F::new(0.61524113149298439947e4) * t555 * t4911 * t1504 * t4915 * t1497;
    let t16283 = F::new(0.14246666666666666666e0) * t204 * t5063 * t1528;
    let t16287 = F::new(0.22911460125803964958e1) * t204 * t148 * t1598 * t1602;
    let t16290 = F::new(0.57895126195293126241e3) * t5008 * t1602 * t1527;
    (t16280, t16283, t16287, t16290)
}
