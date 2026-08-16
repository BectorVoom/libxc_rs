//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1910/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1910(t1294: f64, t8208: f64, t26969: f64, t1775: f64, t1829: f64, t2149: f64, t2152: f64, t27008: f64, t27011: f64, t27025: f64, t29111: f64, t29119: f64, t29124: f64, t29129: f64, t29132: f64, t29136: f64, t29141: f64, t5246: f64, t7602: f64, t7643: f64, t7645: f64, t7648: f64, t7651: f64, t7654: f64, t7659: f64, t7662: f64, t7666: f64, t8198: f64, t8205: f64, t8217: f64) -> (f64, f64) {
    let t29148 = t8208 * t1294;
    let t29149 = t26969 * t29148;
    let t29154 = -0.4336814094102599731e0_f64 * t2149 * t29111 - 0.4336814094102599731e0_f64 * t8205 * t7666 - 0.4336814094102599731e0_f64 * t7648 * t8217 + 0.8673628188205199462e0_f64 * t7643 * t29119 - 0.4336814094102599731e0_f64 * t7659 * t29124 - 0.4336814094102599731e0_f64 * t29129 * t7662 - 0.4336814094102599731e0_f64 * t29132 * t2152 + 0.8673628188205199462e0_f64 * t29136 * t7645 - 0.65854491829355115987e0_f64 * t27008 * t1829 + 0.8673628188205199462e0_f64 * t29141 * t7654 - 0.65854491829355115987e0_f64 * t7602 * t5246 - 0.65854491829355115987e0_f64 * t27011 * t1775 - 0.26020884564615598386e1_f64 * t7651 * t29149 - 0.8673628188205199462e0_f64 * t27025 * t8198;
    (t29149, t29154)
}
