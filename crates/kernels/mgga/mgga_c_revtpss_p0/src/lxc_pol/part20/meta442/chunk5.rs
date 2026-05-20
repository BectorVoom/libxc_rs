//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1692/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1692<F: Float>(t10298: F, t10301: F, t10309: F, t10310: F, t10313: F, t10410: F, t2242: F, t2247: F, t2248: F, t2315: F, t45953: F, t45955: F, t45958: F, t45963: F, t45972: F, t45973: F, t45979: F, t46034: F, t46119: F, t603: F, t644: F, t91: F) -> F {
    let t46123 = t45953 * t91 - F::new(16.0) * t45955 * t644 + F::new(120.0) * t45958 * t2248 - F::new(24.0) * t10298 * t2315 - F::new(480.0) * t45963 * t10310 + F::new(240.0) * t10301 * t10313 - F::new(16.0) * t2242 * t10410 + F::new(840.0) * t45972 * t45973 - F::new(720.0) * t10309 * t2248 * t2315 + F::new(60.0) * t2247 * t45979 + F::new(80.0) * t2247 * t644 * t10410 - F::new(4.0) * t603 * (t46034 + t46119);
    t46123
}
