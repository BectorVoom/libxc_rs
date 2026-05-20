//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 944/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk944<F: Float>(t521: F, t9342: F, t14: F, t588: F, t2496: F, t4038: F, t123: F, t1330: F) -> (F, F, F, F, F) {
    let t9854 = F::new(24.0) * t9342 * t521;
    let t9855 = t14 * t588;
    let t9856 = t9855 * t521;
    let t9857 = F::new(144.0) * t9856;
    let t9858 = t4038 * t2496;
    let t9859 = F::cast_from(0.51947577317044391276e2_f64) * t9858;
    let t9860 = t1330 * t123;
    (t9854, t9855, t9857, t9859, t9860)
}
