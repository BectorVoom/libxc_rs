//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1398/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1398<F: Float>(t235: F, t46475: F, t239: F, t820: F, t2482: F, t4000: F, t596: F, t72: F, t9940: F, t245: F, t136: F, t4010: F) -> (F, F, F, F) {
    let t47201 = t46475 * t235;
    let t47203 = t820 * t47201 * t239;
    let t47215 = t2482 * t4000 * t596;
    let t47247 = t9940 * t72;
    let t47248 = t47247 * t245;
    let t47273 = t4010 * t136;
    (t47203, t47215, t47248, t47273)
}
