//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2149/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2149<F: Float>(t106497: F, t106543: F, t106588: F, t106636: F, t27375: F, t63185: F, t11064: F, t1544: F, t27384: F, t105923: F, t106481: F, t106516: F, t106610: F, t1583: F, t18392: F, t18498: F, t1940: F, t1963: F, t198: F, t207: F, t2403: F, t25206: F, t25440: F, t25445: F, t27158: F, t29598: F, t4343: F, t4433: F, t4541: F, t5962: F, t6075: F, t7087: F, t7091: F, t77408: F, t7783: F, t890: F, t892: F, t98722: F, t99555: F) -> (F, F) {
    let t106638 = t106497 + t106543 + t106588 + t106636;
    let t107793 = t63185 * t27375;
    let t107805 = t11064 * t1544 * t27384;
    let t107820 = F::cast_from(12.0_f64) * t4541 * t7783 * t4433 - F::cast_from(2.0_f64) * t1940 * t99555 * t1583 + F::cast_from(6.0_f64) * t2403 * t7783 * t4343 + F::cast_from(3.0_f64) * t2403 * t1963 * t18392 + F::cast_from(4.0_f64) * t1940 * t98722 * t27384 - t1940 * t106516 * t890 - F::cast_from(6.0_f64) * t4541 * t7091 * t77408 - F::cast_from(12.0_f64) * t27158 * t107793 - t1940 * t25440 * t6075 + F::cast_from(2.0_f64) * t1940 * t25445 * t106610 + t198 * t207 * t106481 * t892 + F::cast_from(12.0_f64) * t25206 * t107805 + F::cast_from(3.0_f64) * t2403 * t7087 * t5962 + F::cast_from(12.0_f64) * t4541 * t1963 * t18498 - F::cast_from(6.0_f64) * t2403 * t25440 * t29598 - F::cast_from(3.0_f64) * t2403 * t7091 * t105923;
    (t106638, t107820)
}
