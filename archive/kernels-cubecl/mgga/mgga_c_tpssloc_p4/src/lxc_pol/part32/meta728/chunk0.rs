//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2365/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2365<F: Float>(t100908: F, t100915: F, t100917: F, t100921: F, t100924: F, t100927: F, t100929: F, t100932: F, t100934: F, t100936: F, t100938: F, t100941: F, t1458: F, t20176: F, t20181: F, t24972: F, t27921: F, t4072: F, t5376: F, t5456: F, t85416: F, t96311: F, t96334: F) -> F {
    let t105128 = F::cast_from(27.0_f64) * t24972 * t20181 + t100908 + F::cast_from(27.0_f64) * t96311 * t1458 + t100915 + F::cast_from(27.0_f64) * t85416 * t5456 + t100917 + t100921 + t100924 + t100927 + t100929 + t100932 + F::cast_from(54.0_f64) * t96334 * t5376 + F::cast_from(27.0_f64) * t27921 * t4072 + F::cast_from(54.0_f64) * t24972 * t20176 + t100934 + t100936 + t100938 + t100941;
    t105128
}
