//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 641/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk641<F: Float>(t218: F, t7021: F, t816: F, t1941: F, t228: F, t802: F, t240: F, t64: F, t234: F, t243: F, t807: F, t1945: F, t786: F) -> (F, F, F, F, F, F, F) {
    let t7023 = t7021 * t218 * t816;
    let t7024 = F::new(7.0) / F::new(288.0) * t7023;
    let t7025 = t1941 * t228;
    let t7026 = t7025 * t802;
    let t7028 = t64 * t240;
    let t7030 = t234 * t7028 * t243;
    let t7031 = t807 * t7030;
    let t7032 = F::cast_from(0.14291339372689912324e-4_f64) * t7031;
    let t7033 = t786 * t1945;
    (t7024, t7025, t7026, t7028, t7030, t7032, t7033)
}
