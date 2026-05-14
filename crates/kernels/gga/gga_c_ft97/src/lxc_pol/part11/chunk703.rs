//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 703/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk703<F: Float>(t274: F, t668: F, t505: F, t10326: F, t801: F, t9525: F, t231: F, t123: F, t805: F, t9606: F, t278: F, t2417: F, t808: F, t194: F, t197: F, t8991: F) -> (F, F, F, F, F, F, F, F) {
    let t10327 = t274 * t668;
    let t10328 = t10327 * t505;
    let t10329 = t10326 * t10328;
    let t10333 = t9525 * t801 * t274;
    let t10334 = t231 * t10333;
    let t10339 = t123 / t805 / t9606;
    let t10340 = t9525 * t278;
    let t10343 = t808 * t2417;
    let t10355 = t8991 / t197 / t194;
    (t10327, t10328, t10329, t10334, t10339, t10340, t10343, t10355)
}
