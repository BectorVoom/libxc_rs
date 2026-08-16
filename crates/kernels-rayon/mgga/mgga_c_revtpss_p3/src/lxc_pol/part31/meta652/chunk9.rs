//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2173/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2173(t27375: f64, t63185: f64, t11064: f64, t1544: f64, t27384: f64, t105923: f64, t106481: f64, t106516: f64, t106610: f64, t1583: f64, t18392: f64, t18498: f64, t1940: f64, t1963: f64, t198: f64, t207: f64, t2403: f64, t25206: f64, t25440: f64, t25445: f64, t27158: f64, t29598: f64, t4343: f64, t4433: f64, t4541: f64, t5962: f64, t6075: f64, t7087: f64, t7091: f64, t77408: f64, t7783: f64, t890: f64, t892: f64, t98722: f64, t99555: f64) -> f64 {
    let t107793 = t63185 * t27375;
    let t107805 = t11064 * t1544 * t27384;
    let t107820 = 12.0_f64 * t4541 * t7783 * t4433 - 2.0_f64 * t1940 * t99555 * t1583 + 6.0_f64 * t2403 * t7783 * t4343 + 3.0_f64 * t2403 * t1963 * t18392 + 4.0_f64 * t1940 * t98722 * t27384 - t1940 * t106516 * t890 - 6.0_f64 * t4541 * t7091 * t77408 - 12.0_f64 * t27158 * t107793 - t1940 * t25440 * t6075 + 2.0_f64 * t1940 * t25445 * t106610 + t198 * t207 * t106481 * t892 + 12.0_f64 * t25206 * t107805 + 3.0_f64 * t2403 * t7087 * t5962 + 12.0_f64 * t4541 * t1963 * t18498 - 6.0_f64 * t2403 * t25440 * t29598 - 3.0_f64 * t2403 * t7091 * t105923;
    t107820
}
