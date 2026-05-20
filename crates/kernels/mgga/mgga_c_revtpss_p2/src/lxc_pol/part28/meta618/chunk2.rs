//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2167/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2167<F: Float>(t14741: F, t1945: F, t807: F, t10886: F, t4416: F, t7028: F, t27221: F, t50789: F, t50931: F, t1549: F, t92968: F, t14697: F, t25270: F) -> (F, F, F, F, F, F) {
    let t99041 = t807 * t1945 * t14741;
    let t99042 = F::cast_from(0.11433071498151929859e-3_f64) * t99041;
    let t99044 = t10886 * t7028 * t4416;
    let t99046 = t27221 * t50789;
    let t99048 = t27221 * t50931;
    let t99050 = t92968 * t1549;
    let t99052 = t25270 * t14697;
    (t99042, t99044, t99046, t99048, t99050, t99052)
}
