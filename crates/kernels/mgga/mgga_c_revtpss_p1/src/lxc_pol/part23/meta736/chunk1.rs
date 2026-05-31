//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2511/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2511<F: Float>(t50773: F, t4398: F, t9323: F, t4302: F, t9586: F, t10612: F, t4311: F, t14440: F, t2398: F, t14322: F, t2626: F, t9425: F) -> (F, F, F, F, F, F, F) {
    let t50774 = F::cast_from(0.40656002247428262579e-3_f64) * t50773;
    let t50852 = t4398 * t9323;
    let t50856 = t4302 * t9586;
    let t50865 = t4311 * t10612;
    let t50866 = F::cast_from(12.0_f64) * t50865;
    let t50873 = t2398 * t14440;
    let t50874 = F::cast_from(12.0_f64) * t50873;
    let t50883 = t14322 * t2626;
    let t50884 = F::cast_from(0.35089341735807877242e1_f64) * t50883;
    let t50888 = t4398 * t9425;
    (t50774, t50852, t50856, t50866, t50874, t50884, t50888)
}
