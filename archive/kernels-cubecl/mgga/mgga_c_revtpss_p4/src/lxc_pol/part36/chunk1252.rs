//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1252/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1252<F: Float>(t1561: F, t93048: F, t10886: F, t4416: F, t7028: F, t1549: F, t92968: F, t2689: F, t27239: F, t14760: F, t93015: F, t2435: F, t27334: F) -> (F, F, F, F, F, F) {
    let t99035 = t93048 * t1561;
    let t99044 = t10886 * t7028 * t4416;
    let t99050 = t92968 * t1549;
    let t99091 = t2689 * t27239;
    let t99113 = t93015 * t14760;
    let t99166 = t2435 * t27334;
    (t99035, t99044, t99050, t99091, t99113, t99166)
}
