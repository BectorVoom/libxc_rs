//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 901/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk901<F: Float>(t10534: F, t126: F, t83: F, t10496: F, t10501: F, t10502: F, t10506: F, t10509: F, t10512: F, t135: F, t144: F, t192: F, t2718: F, t5087: F, t5091: F, t5130: F, t5139: F, t5141: F, t5144: F, t5148: F, t5154: F, t5165: F) -> (F, F, F) {
    let t10535 = t10534 * t126;
    let t10536 = t83 * t10535;
    let t10537 = 2.0 * t10496 * t135 * t144 * t5165 + 6.0 * t10502 * t135 * t192 + 18.0 * t10506 * t2718 + 18.0 * t10509 * t2718 - t10501 + t10512 + t10536 + t5087 + t5091 - t5130 - t5139 - t5141 - t5144 - t5148 - t5154;
    (t10535, t10536, t10537)
}
