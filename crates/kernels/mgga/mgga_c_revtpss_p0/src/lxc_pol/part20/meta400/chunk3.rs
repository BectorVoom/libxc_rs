//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1486/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1486<F: Float>(t367: F, t371: F, t373: F, t9291: F, t1058: F, t11907: F, t3197: F, t3201: F, t11962: F, t3231: F, t11973: F, t11904: F) -> (F, F, F, F, F, F, F) {
    let t42121 = F::cast_from(0.14820648238345094262e-3_f64) * t367 * t371 * t9291 * t373;
    let t42122 = t11907 * t1058;
    let t42124 = t3197 * t3201;
    let t42139 = t11962 * t1058;
    let t42141 = t3231 * t3201;
    let t42146 = t11973 * t1058;
    let t42149 = t11904 * t1058;
    (t42121, t42122, t42124, t42139, t42141, t42146, t42149)
}
