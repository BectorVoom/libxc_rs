//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2429/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2429<F: Float>(t20: F, t2237: F, t12: F, t14: F, t27: F, t10285: F, t596: F, t10293: F, t592: F, t25: F, t40649: F, t10308: F, t599: F) -> (F, F, F, F, F, F) {
    let t45941 = F::new(840.0) * t20 * t2237;
    let t45944 = F::new(360.0) * t12 * t14 * t27;
    let t45945 = t10285 * t596;
    let t45949 = t592 * t10293;
    let t45952 = F::new(3024.0) * t25 * t40649;
    let t45963 = t599 * t10308;
    (t45941, t45944, t45945, t45949, t45952, t45963)
}
