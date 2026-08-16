//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1833/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1833(t10090: f64, t1883: f64, t213: f64, t22912: f64, t4114: f64, t46476: f64, t47961: f64, t546: f64, t5767: f64, t74901: f64, t820: f64, t86374: f64, t86377: f64, t86381: f64, t86552: f64, t91922: f64, t92177: f64, t92182: f64, t92219: f64) -> f64 {
    let t92317 = 0.65854491829355115987e0_f64 * t213 * t546 * t92219 - 0.44178176337912614788e-3_f64 * t47961 - 0.26341796731742046395e1_f64 * t820 * t5767 * t22912 + 0.15805078039045227836e2_f64 * t820 * t46476 * t92182 - 0.23707617058567841754e2_f64 * t820 * t10090 * t91922 + 0.69394917116090352835e-2_f64 * t74901 + 0.39029762157531132076e-1_f64 * t86374 - 0.26341796731742046395e1_f64 * t820 * t86552 * t1883 + 0.92196288561097162379e1_f64 * t820 * t4114 * t92177 - 0.11708928647259339623e0_f64 * t86377 + 0.23417857294518679245e0_f64 * t86381;
    t92317
}
