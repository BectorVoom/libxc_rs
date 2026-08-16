//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 858/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk858<F: Float>(t197: F, t509: F, t1270: F, t1844: F, t64: F, t789: F, t112: F, t2023: F, t641: F, t629: F, t98: F, t99: F) -> (F, F, F, F, F, F, F) {
    let t7309 = t197 * t509;
    let t7383 = t1844 * t1270;
    let t7585 = t64 * t789;
    let t7587 = F::cast_from(154.0_f64) / F::cast_from(27.0_f64) * t7585 * t112;
    let t7588 = t2023 * t641;
    let t7593 = t629 * t629;
    let t7594 = F::cast_from(1.0_f64) / t7593;
    let t7612 = t99 * t98;
    (t7309, t7383, t7585, t7587, t7588, t7594, t7612)
}
