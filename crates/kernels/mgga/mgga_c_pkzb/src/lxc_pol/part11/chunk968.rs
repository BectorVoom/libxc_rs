//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 968/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk968<F: Float>(t139: F, t2177: F, t1516: F, t490: F, t4994: F, t1542: F, t1626: F, t1497: F, t1504: F, t4911: F, t4915: F, t555: F, t1528: F, t204: F, t5063: F, t148: F, t1598: F, t1602: F) -> (F, F, F, F, F, F) {
    let t16250 = 1.0 / t139 / t2177;
    let t16273 = 8.0 * t1516 * t4994 * t490;
    let t16275 = 120.0 * t1542 * t1626;
    let t16280 = 0.61524113149298439947e4 * t555 * t4911 * t1504 * t4915 * t1497;
    let t16283 = 0.14246666666666666666e0 * t204 * t5063 * t1528;
    let t16287 = 0.22911460125803964958e1 * t204 * t148 * t1598 * t1602;
    (t16250, t16273, t16275, t16280, t16283, t16287)
}
