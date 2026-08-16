//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 964/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk964(t10534: f64, t126: f64, t83: f64, t10496: f64, t10501: f64, t10502: f64, t10506: f64, t10509: f64, t10512: f64, t135: f64, t144: f64, t192: f64, t2718: f64, t5087: f64, t5091: f64, t5130: f64, t5139: f64, t5141: f64, t5144: f64, t5148: f64, t5154: f64, t5165: f64) -> (f64, f64, f64) {
    let t10535 = t10534 * t126;
    let t10536 = t83 * t10535;
    let t10537 = 2.0_f64 * t10496 * t135 * t144 * t5165 + 6.0_f64 * t10502 * t135 * t192 + 18.0_f64 * t10506 * t2718 + 18.0_f64 * t10509 * t2718 - t10501 + t10512 + t10536 + t5087 + t5091 - t5130 - t5139 - t5141 - t5144 - t5148 - t5154;
    (t10535, t10536, t10537)
}
