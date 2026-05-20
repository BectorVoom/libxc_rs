//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1833/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1833<F: Float>(t10090: F, t1883: F, t213: F, t22912: F, t4114: F, t46476: F, t47961: F, t546: F, t5767: F, t74901: F, t820: F, t86374: F, t86377: F, t86381: F, t86552: F, t91922: F, t92177: F, t92182: F, t92219: F) -> F {
    let t92317 = F::cast_from(0.65854491829355115987e0_f64) * t213 * t546 * t92219 - F::cast_from(0.44178176337912614788e-3_f64) * t47961 - F::cast_from(0.26341796731742046395e1_f64) * t820 * t5767 * t22912 + F::cast_from(0.15805078039045227836e2_f64) * t820 * t46476 * t92182 - F::cast_from(0.23707617058567841754e2_f64) * t820 * t10090 * t91922 + F::cast_from(0.69394917116090352835e-2_f64) * t74901 + F::cast_from(0.39029762157531132076e-1_f64) * t86374 - F::cast_from(0.26341796731742046395e1_f64) * t820 * t86552 * t1883 + F::cast_from(0.92196288561097162379e1_f64) * t820 * t4114 * t92177 - F::cast_from(0.11708928647259339623e0_f64) * t86377 + F::cast_from(0.23417857294518679245e0_f64) * t86381;
    t92317
}
