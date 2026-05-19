//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1012/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1012<F: Float>(t44088: F, t3039: F, t5774: F, t3277: F, t13009: F, t5782: F, t1457: F, t43240: F, t6060: F, t13158: F, t15766: F, t41425: F) -> (F, F, F, F, F, F) {
    let t44089 = F::cast_from(0.15976219147466979032e-1_f64) * t44088;
    let t44090 = t3039 * t5774;
    let t44092 = F::cast_from(0.16683561977530199113e1_f64) * t3277 * t44090;
    let t44093 = t5782 * t13009;
    let t44097 = F::cast_from(0.21450293971110256001e1_f64) * t6060 * t1457 * t43240;
    let t44099 = F::cast_from(0.21450293971110256001e1_f64) * t15766 * t13158;
    let t44106 = F::cast_from(0.1022478025437886658e1_f64) * t41425;
    (t44089, t44092, t44093, t44097, t44099, t44106)
}
