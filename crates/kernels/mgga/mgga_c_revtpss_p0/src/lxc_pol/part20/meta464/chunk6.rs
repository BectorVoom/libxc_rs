//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1770/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1770<F: Float>(t10008: F, t545: F, t689: F, t869: F, t4093: F, t9292: F, t10065: F, t10073: F, t1432: F, t1433: F, t39497: F, t1385: F) -> (F, F, F, F, F) {
    let t47387 = t689 * t869 * t545 * t10008;
    let t47389 = t9292 * t4093;
    let t47391 = t10073 * t10065;
    let t47395 = F::cast_from(0.10118827226026589797e0_f64) * t1432 * t1433 * t39497;
    let t47396 = t1385 * t10008;
    (t47387, t47389, t47391, t47395, t47396)
}
