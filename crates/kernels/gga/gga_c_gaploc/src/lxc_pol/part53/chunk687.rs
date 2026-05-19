//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 687/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk687<F: Float>(t12383: F, t12386: F, t12397: F, t12400: F, t12412: F, t10318: F, t2321: F, t9074: F, t3158: F, t993: F, t2268: F, t10268: F, t4261: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12787 = F::new(9.0) / F::new(256.0) * t12383;
    let t12788 = F::new(9.0) / F::new(8192.0) * t12386;
    let t12789 = F::new(3.0) / F::new(8192.0) * t12397;
    let t12790 = F::new(3.0) / F::new(256.0) * t12400;
    let t12791 = F::new(2.0) * t12412;
    let t12797 = t10318 * t2321;
    let t12798 = t9074 * t12797;
    let t12799 = F::cast_from(0.23712505529730124666e-2_f64) * t12798;
    let t12800 = t3158 * t993;
    let t12802 = F::cast_from(0.19918504644973304719e0_f64) * t2268 * t12800;
    let t12803 = t4261 * t10268;
    (t12787, t12788, t12789, t12790, t12791, t12797, t12799, t12800, t12802, t12803)
}
