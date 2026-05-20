//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 560/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk560<F: Float>(t1024: F, t1087: F, t1647: F, t1685: F, t1689: F, t1692: F, t342: F, t381: F) -> F {
    let t1695 = F::cast_from(0.65854491829355115987e0_f64) * t1647 * t381 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t1685 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t1689 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t1692;
    t1695
}
