//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 965/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk965<F: Float>(t1276: F, t2148: F, t29127: F, t5412: F, t1032: F, t1811: F, t7642: F, t1294: F, t8208: F, t26969: F, t1775: F, t1829: F, t2149: F, t2152: F, t27008: F, t27011: F, t27025: F, t29111: F, t29119: F, t29124: F, t5246: F, t7602: F, t7643: F, t7645: F, t7648: F, t7651: F, t7654: F, t7659: F, t7662: F, t7666: F, t8198: F, t8205: F, t8217: F) -> (F, F) {
    let t29129 = t2148 * t29127 * t1276;
    let t29132 = t2148 * t5412;
    let t29135 = t1811 * t1032;
    let t29136 = t7642 * t29135;
    let t29141 = t2148 * t29135;
    let t29148 = t8208 * t1294;
    let t29149 = t26969 * t29148;
    let t29154 = -F::cast_from(0.4336814094102599731e0_f64) * t2149 * t29111 - F::cast_from(0.4336814094102599731e0_f64) * t8205 * t7666 - F::cast_from(0.4336814094102599731e0_f64) * t7648 * t8217 + F::cast_from(0.8673628188205199462e0_f64) * t7643 * t29119 - F::cast_from(0.4336814094102599731e0_f64) * t7659 * t29124 - F::cast_from(0.4336814094102599731e0_f64) * t29129 * t7662 - F::cast_from(0.4336814094102599731e0_f64) * t29132 * t2152 + F::cast_from(0.8673628188205199462e0_f64) * t29136 * t7645 - F::cast_from(0.65854491829355115987e0_f64) * t27008 * t1829 + F::cast_from(0.8673628188205199462e0_f64) * t29141 * t7654 - F::cast_from(0.65854491829355115987e0_f64) * t7602 * t5246 - F::cast_from(0.65854491829355115987e0_f64) * t27011 * t1775 - F::cast_from(0.26020884564615598386e1_f64) * t7651 * t29149 - F::cast_from(0.8673628188205199462e0_f64) * t27025 * t8198;
    (t29135, t29154)
}
