//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 763/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk763<F: Float>(t10326: F, t10328: F, t274: F, t801: F, t9525: F, t231: F, t123: F, t805: F, t9606: F, t278: F, t2417: F, t808: F) -> (F, F, F, F, F) {
    let t10329 = t10326 * t10328;
    let t10333 = t9525 * t801 * t274;
    let t10334 = t231 * t10333;
    let t10339 = t123 / t805 / t9606;
    let t10340 = t9525 * t278;
    let t10343 = t808 * t2417;
    (t10329, t10334, t10339, t10340, t10343)
}
