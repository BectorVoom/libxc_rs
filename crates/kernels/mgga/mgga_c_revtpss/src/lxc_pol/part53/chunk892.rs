//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 892/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk892<F: Float>(t25399: F, t4481: F, t1580: F, t213: F, t25322: F, t25362: F, t25364: F, t25366: F, t25368: F, t25371: F, t25379: F, t25391: F, t257: F, t27199: F, t27300: F, t27303: F, t27313: F, t27317: F, t27322: F, t7070: F, t7079: F) -> F {
    let t27325 = t25399 * t4481;
    let t27329 = -F::cast_from(0.26020884564615598386e1_f64) * t7070 * t27300 - t25362 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t27303 * t257 + F::cast_from(0.4336814094102599731e0_f64) * t27199 * t7079 - t25364 - F::cast_from(0.12851425765524037203e-1_f64) * t25366 - F::cast_from(0.12851425765524037203e-1_f64) * t25368 + t25371 - F::cast_from(0.8673628188205199462e0_f64) * t25391 * t27313 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t27317 - F::cast_from(0.14456046980341999104e-1_f64) * t25379 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t27322 - F::cast_from(0.9757440539382783019e-2_f64) * t27325 - F::cast_from(0.65854491829355115987e0_f64) * t25322 * t1580;
    t27329
}
