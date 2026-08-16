//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2239/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2239(t14365: f64, t14436: f64, t14468: f64, t14749: f64, t14767: f64, t1940: f64, t1963: f64, t198: f64, t207: f64, t2394: f64, t2403: f64, t2408: f64, t25206: f64, t25445: f64, t27368: f64, t27384: f64, t4433: f64, t4541: f64, t61155: f64, t61182: f64, t63164: f64, t7087: f64, t7091: f64, t7783: f64, t892: f64, t92742: f64, t93404: f64, t98722: f64, t98759: f64, t98779: f64, t98786: f64, t99536: f64) -> f64 {
    let t100858 = t14436 * t14365;
    let t100882 = 12.0_f64 * t4541 * t7087 * t4433 + 2.0_f64 * t1940 * t98722 * t2408 - 6.0_f64 * t4541 * t7091 * t98759 - 6.0_f64 * t2403 * t27368 * t14365 + 4.0_f64 * t1940 * t93404 * t27384 + t198 * t207 * t99536 * t892 + 6.0_f64 * t2403 * t25445 * t61155 + 4.0_f64 * t1940 * t25445 * t63164 + 12.0_f64 * t25206 * t100858 - 6.0_f64 * t1940 * t92742 * t98786 + 3.0_f64 * t2403 * t1963 * t14468 + 6.0_f64 * t4541 * t7783 * t2394 - 6.0_f64 * t2403 * t7091 * t61182 + 2.0_f64 * t1940 * t25445 * t98779 + 12.0_f64 * t4541 * t1963 * t14749 + 6.0_f64 * t4541 * t1963 * t14767;
    t100882
}
