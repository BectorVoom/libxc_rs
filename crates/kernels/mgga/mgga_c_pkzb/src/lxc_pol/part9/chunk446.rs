//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 446/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk446<F: Float>(t1560: F, t1820: F, t291: F, t197: F, t1281: F, t204: F, t208: F) -> (F, F, F) {
    let t1821 = t1560 + t1820;
    let t1823 = F::new(1.0) / t291;
    let t1824 = t197 * t1823;
    let t1830 = t204 * t1281 * t208;
    (t1821, t1824, t1830)
}
