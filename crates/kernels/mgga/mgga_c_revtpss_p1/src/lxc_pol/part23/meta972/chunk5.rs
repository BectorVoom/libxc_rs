//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3296/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3296<F: Float>(t1385: F, t22964: F, t1399: F, t14230: F, t14255: F, t1883: F, t22009: F, t49238: F, t49256: F, t49274: F, t5675: F, t5745: F, t6844: F, t74886: F, t75141: F, t75145: F, t75147: F, t820: F, t86470: F) -> F {
    let t86552 = t1385 * t22964;
    let t86556 = F::cast_from(0.11708928647259339623e0_f64) * t75141 + F::cast_from(0.19514881078765566038e-2_f64) * t49238 + F::cast_from(0.43902994552903410656e-1_f64) * t75145 - F::cast_from(0.43902994552903410656e-1_f64) * t75147 + F::cast_from(0.11853808529283920877e2_f64) * t5745 * t22009 * t14230 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t14255 * t6844 - F::cast_from(0.78059524315062264152e-1_f64) * t49256 + F::cast_from(0.11853808529283920877e2_f64) * t5745 * t86470 * t5675 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t74886 * t1883 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t86552 * t1399 - t49274;
    t86556
}
