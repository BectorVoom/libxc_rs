//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 892/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk892<F: Float>(t15499: F, t3601: F, t2679: F, t28640: F, t10827: F, t3005: F, t9800: F, t2676: F, t36782: F, t3621: F, t9796: F, t1029: F) -> (F, F, F, F, F) {
    let t45209 = t15499 * t3601;
    let t45211 = t28640 * t45209 * t2679;
    let t45212 = F::cast_from(0.23005755572352449806e1_f64) * t45211;
    let t45214 = t9800 * t3005 * t10827;
    let t45215 = F::cast_from(0.19171462976960374838e1_f64) * t45214;
    let t45217 = F::cast_from(0.27805936629216998521e0_f64) * t36782 * t2676;
    let t45219 = t9796 * t3621 * t2679;
    let t45222 = t9796 * t1029 * t10827;
    (t45212, t45215, t45217, t45219, t45222)
}
