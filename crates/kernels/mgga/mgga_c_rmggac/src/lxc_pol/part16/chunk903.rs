//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 903/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk903<F: Float>(t1818: F, t236: F, t3351: F, t40168: F, t498: F, t10018: F, t7255: F, t1910: F, t495: F, t7230: F, t7231: F, t9210: F, t321: F, t7248: F, t333: F, t511: F) -> (F, F, F, F, F, F) {
    let t47621 = t3351 * t40168 * t236 * t1818 * t498;
    let t47623 = t7255 * t10018;
    let t47629 = t7230 * t7231 * t236 * t1910 * t495;
    let t47634 = t3351 * t9210 * t236 * t1910 * t498;
    let t47639 = t3351 * t7248 * t236 * t1910 * t321;
    let t47644 = t3351 * t7231 * t511 * t1910 * t333;
    (t47621, t47623, t47629, t47634, t47639, t47644)
}
