//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 842/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk842<F: Float>(t581: F, t9011: F, t1733: F, t5244: F, t5279: F, t5297: F, t5385: F, t5405: F, t580: F, t6968: F, t6988: F, t6995: F, t6998: F, t7009: F, t8996: F, t9000: F, t9005: F, t9008: F) -> (F, F) {
    let t9012 = t581 * t9011;
    let t9017 = -F::cast_from(0.22675591804667994221e-1_f64) * t5297 - F::cast_from(0.34299214494455789578e-2_f64) * t5244 * t8996 - F::cast_from(0.85748036236139473945e-2_f64) * t5279 * t9000 + F::cast_from(0.17149607247227894789e-2_f64) * t1733 * t9005 + F::cast_from(0.40015750243531754507e-2_f64) * t9008 - F::cast_from(0.56688979511669985553e-2_f64) * t5385 - t580 * t9012 / F::new(48.0) - t5405 + t6968 - F::cast_from(0.45351183609335988442e-1_f64) * t6988 - F::cast_from(0.11337795902333997111e-1_f64) * t6995 - t6998 + t7009;
    (t9012, t9017)
}
