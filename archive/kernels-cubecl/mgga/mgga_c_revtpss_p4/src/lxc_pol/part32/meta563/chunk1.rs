//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1885/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1885<F: Float>(t14741: F, t1945: F, t807: F, t10886: F, t4416: F, t7028: F, t1549: F, t92968: F, t93001: F, t10778: F, t1941: F, t93016: F) -> (F, F, F, F, F, F) {
    let t99041 = t807 * t1945 * t14741;
    let t99044 = t10886 * t7028 * t4416;
    let t99050 = t92968 * t1549;
    let t99058 = F::cast_from(0.1219527626469539185e-2_f64) * t93001;
    let t99062 = t1941 * t10778;
    let t99065 = F::cast_from(0.18071592998981862717e-4_f64) * t93016;
    (t99041, t99044, t99050, t99058, t99062, t99065)
}
