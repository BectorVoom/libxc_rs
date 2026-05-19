//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1242/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1242<F: Float>(t17403: F, t20717: F, t20749: F, t20752: F, t20754: F, t30314: F, t30316: F, t30319: F, t30322: F, t30324: F, t30326: F, t30328: F, t30331: F, t30338: F, t30342: F, t30346: F, t30350: F, t30353: F, t30356: F) -> F {
    let t30541 = F::cast_from(0.247573125e0_f64) * t30314 + F::cast_from(0.247573125e0_f64) * t30316 + F::new(0.82524375e-1) * t30319 - F::cast_from(0.485484375e1_f64) * t30322 + F::new(0.58258125e1) * t30324 - F::new(0.3883875e1) * t30326 - F::new(0.3883875e1) * t30328 - F::new(0.1294625e1) * t30331 + t20717 + t20749 + t20752 - F::new(0.22076e1) * t20754 + t17403 + F::new(0.745065e0) * t30338 + F::new(0.248355e0) * t30342 + F::new(0.248355e0) * t30346 + F::new(0.745065e0) * t30350 - F::new(0.49671e0) * t30353 - F::new(0.16557e0) * t30356;
    t30541
}
