//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 901/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk901<F: Float>(t27242: F, t27264: F, t233: F, t1957: F, t1956: F, t27183: F, t27187: F, t27189: F, t27192: F, t27196: F, t27199: F, t27203: F, t27207: F, t27214: F, t27217: F, t4487: F, t4534: F, t7053: F, t7067: F, t7070: F, t7073: F, t7779: F, t887: F) -> (F, F, F) {
    let t27265 = t27242 + t27264;
    let t27266 = t233 * t27265;
    let t27267 = t1957 * t27266;
    let t27272 = -F::new(0.65854491829355115987e0) * t7053 * t4534 + F::new(0.8673628188205199462e0) * t7070 * t27183 + F::new(0.12851425765524037203e-1) * t27187 - F::new(0.65854491829355115987e0) * t27189 * t887 - F::new(0.72280234901709995518e-2) * t27192 - F::new(0.54878743191129263322e-2) * t27196 + F::new(0.8673628188205199462e0) * t27199 * t7073 + F::new(0.9757440539382783019e-2) * t27203 + F::new(0.4336814094102599731e0) * t7070 * t27207 - F::new(0.4336814094102599731e0) * t7067 * t7779 + F::new(0.72280234901709995518e-2) * t27214 - F::new(0.12851425765524037203e-1) * t27217 - F::new(0.4336814094102599731e0) * t1956 * t27267 + F::new(0.13170898365871023197e1) * t7053 * t4487;
    (t27265, t27267, t27272)
}
