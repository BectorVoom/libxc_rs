//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 917/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk917<F: Float>(t2648: F, t6966: F, t1733: F, t5244: F, t5297: F, t5299: F, t5301: F, t5315: F, t5379: F, t5382: F, t5385: F, t5405: F, t6941: F, t6946: F, t6958: F, t6963: F) -> F {
    let t6968 = F::cast_from(0.20007875121765877254e-2_f64) * t6966 * t2648;
    let t6969 = F::cast_from(0.17149607247227894789e-2_f64) * t1733 * t6941 - F::cast_from(0.34299214494455789578e-2_f64) * t5244 * t6946 - F::cast_from(0.45351183609335988442e-1_f64) * t5297 + F::cast_from(0.40015750243531754508e-2_f64) * t5299 - F::cast_from(0.20007875121765877254e-1_f64) * t5301 + F::cast_from(0.10003937560882938627e-2_f64) * t5315 + F::cast_from(0.10003937560882938627e-2_f64) * t5379 - F::cast_from(0.20007875121765877254e-2_f64) * t5382 - F::cast_from(0.11337795902333997111e-1_f64) * t5385 - t5405 + F::cast_from(0.17149607247227894789e-2_f64) * t1733 * t6958 + F::cast_from(0.85748036236139473944e-3_f64) * t1733 * t6963 + t6968;
    t6969
}
