//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 935/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk935<F: Float>(t3978: F, t5622: F, t1885: F, t3930: F, t1353: F, t1868: F, t4012: F, t828: F, t3826: F, t187: F, t5566: F, t1856: F, t72: F) -> (F, F, F, F, F, F, F) {
    let t5623 = t3978 * t5622;
    let t5625 = t3930 * t1885;
    let t5627 = t1868 * t1353;
    let t5629 = t4012 * t828 * t5627;
    let t5632 = F::cast_from(0.18311447306006545054e-3_f64) * t3826;
    let t5634 = F::cast_from(0.19751673498613801407e-1_f64) * t5566 * t187;
    let t5635 = t1856 * t72;
    (t5623, t5625, t5627, t5629, t5632, t5634, t5635)
}
