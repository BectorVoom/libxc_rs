//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 819/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk819<F: Float>(t521: F, t9342: F, t14: F, t588: F, t2496: F, t4038: F, t123: F, t1330: F, t2630: F, t2516: F, t676: F, t3869: F) -> (F, F, F, F, F, F) {
    let t9854 = F::cast_from(24.0_f64) * t9342 * t521;
    let t9855 = t14 * t588;
    let t9856 = t9855 * t521;
    let t9857 = F::cast_from(144.0_f64) * t9856;
    let t9858 = t4038 * t2496;
    let t9859 = F::cast_from(0.51947577317044391276e2_f64) * t9858;
    let t9860 = t1330 * t123;
    let t9861 = t9860 * t2630;
    let t9862 = F::cast_from(0.32530743900905219526e-1_f64) * t9861;
    let t9863 = t676 * t2516;
    let t9865 = F::cast_from(0.16265371950452609763e-1_f64) * t3869 * t9863;
    (t9854, t9857, t9859, t9862, t9863, t9865)
}
