//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 829/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk829<F: Float>(t27242: F, t27264: F, t233: F, t1957: F, t1956: F, t27183: F, t27187: F, t27189: F, t27192: F, t27196: F, t27199: F, t27203: F, t27207: F, t27214: F, t27217: F, t4487: F, t4534: F, t7053: F, t7067: F, t7070: F, t7073: F, t7779: F, t887: F) -> (F, F, F) {
    let t27265 = t27242 + t27264;
    let t27266 = t233 * t27265;
    let t27267 = t1957 * t27266;
    let t27272 = -F::cast_from(0.65854491829355115987e0_f64) * t7053 * t4534 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t27183 + F::cast_from(0.12851425765524037203e-1_f64) * t27187 - F::cast_from(0.65854491829355115987e0_f64) * t27189 * t887 - F::cast_from(0.72280234901709995518e-2_f64) * t27192 - F::cast_from(0.54878743191129263322e-2_f64) * t27196 + F::cast_from(0.8673628188205199462e0_f64) * t27199 * t7073 + F::cast_from(0.9757440539382783019e-2_f64) * t27203 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t27207 - F::cast_from(0.4336814094102599731e0_f64) * t7067 * t7779 + F::cast_from(0.72280234901709995518e-2_f64) * t27214 - F::cast_from(0.12851425765524037203e-1_f64) * t27217 - F::cast_from(0.4336814094102599731e0_f64) * t1956 * t27267 + F::cast_from(0.13170898365871023197e1_f64) * t7053 * t4487;
    (t27265, t27267, t27272)
}
