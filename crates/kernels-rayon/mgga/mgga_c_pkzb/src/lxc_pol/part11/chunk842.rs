//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 842/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk842(t581: f64, t9011: f64, t1733: f64, t5244: f64, t5279: f64, t5297: f64, t5385: f64, t5405: f64, t580: f64, t6968: f64, t6988: f64, t6995: f64, t6998: f64, t7009: f64, t8996: f64, t9000: f64, t9005: f64, t9008: f64) -> (f64, f64) {
    let t9012 = t581 * t9011;
    let t9017 = -0.22675591804667994221e-1_f64 * t5297 - 0.34299214494455789578e-2_f64 * t5244 * t8996 - 0.85748036236139473945e-2_f64 * t5279 * t9000 + 0.17149607247227894789e-2_f64 * t1733 * t9005 + 0.40015750243531754507e-2_f64 * t9008 - 0.56688979511669985553e-2_f64 * t5385 - t580 * t9012 / 48.0_f64 - t5405 + t6968 - 0.45351183609335988442e-1_f64 * t6988 - 0.11337795902333997111e-1_f64 * t6995 - t6998 + t7009;
    (t9012, t9017)
}
