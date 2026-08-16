//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2240/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2240(t15071: f64, t1544: f64, t1583: f64, t18875: f64, t1940: f64, t2403: f64, t2430: f64, t25436: f64, t25440: f64, t27158: f64, t27364: f64, t27368: f64, t27375: f64, t2832: f64, t4343: f64, t4537: f64, t51780: f64, t61102: f64, t61203: f64, t63186: f64, t7087: f64, t7091: f64, t775: f64, t7783: f64, t7847: f64, t890: f64, t92775: f64, t98651: f64, t99555: f64) -> f64 {
    let t100926 = -t15071 * t1940 * t7091 + 3.0_f64 * t1544 * t2403 * t25436 - t1583 * t1940 * t92775 - 6.0_f64 * t18875 * t2403 * t25440 - 2.0_f64 * t1940 * t25440 * t4537 - t1940 * t27368 * t2832 - 2.0_f64 * t1940 * t890 * t99555 + 3.0_f64 * t2403 * t2430 * t7783 - 6.0_f64 * t2403 * t25440 * t27375 + 6.0_f64 * t2403 * t27364 * t775 + 6.0_f64 * t2403 * t4343 * t7087 - 6.0_f64 * t2403 * t61102 * t7091 - 3.0_f64 * t2403 * t61203 * t7091 - 3.0_f64 * t2403 * t7091 * t98651 - 12.0_f64 * t27158 * t63186 + 6.0_f64 * t51780 * t7847;
    t100926
}
