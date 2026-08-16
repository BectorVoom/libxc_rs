//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1044/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1044<F: Float>(t1112: F, t8598: F, t3483: F, t8601: F, t2964: F, t3537: F, t3808: F, t4908: F, t687: F, t4915: F, t1049: F, t1616: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t12046 = t8598 * t1112;
    let t12047 = t8601 * t3483;
    let t12048 = F::cast_from(2.0_f64) * t12047;
    let t12049 = t2964 * t3537;
    let t12050 = t4908 * t3808;
    let t12051 = F::cast_from(2.0_f64) * t12050;
    let t12052 = t3808 * t687;
    let t12053 = t4915 * t12052;
    let t12054 = F::cast_from(6.0_f64) * t12053;
    let t12055 = t3537 * t1049;
    let t12056 = t1616 * t12055;
    (t12046, t12047, t12048, t12049, t12050, t12051, t12052, t12053, t12054, t12055, t12056)
}
