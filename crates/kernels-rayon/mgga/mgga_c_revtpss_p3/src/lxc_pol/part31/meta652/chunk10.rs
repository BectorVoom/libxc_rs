//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2174/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2174(t106554: f64, t106561: f64, t106565: f64, t106625: f64, t1544: f64, t18435: f64, t18838: f64, t18875: f64, t1940: f64, t1963: f64, t2403: f64, t25445: f64, t27364: f64, t27368: f64, t27375: f64, t29705: f64, t29907: f64, t4537: f64, t4541: f64, t50080: f64, t5966: f64, t6079: f64, t7087: f64, t7091: f64, t77425: f64, t77441: f64, t775: f64, t92742: f64, t93404: f64) -> f64 {
    let t107867 = 4.0_f64 * t106554 * t1940 * t25445 + 6.0_f64 * t106561 * t2403 * t25445 - 6.0_f64 * t106565 * t1940 * t92742 - 6.0_f64 * t106625 * t2403 * t7091 + 6.0_f64 * t1544 * t2403 * t27364 + 6.0_f64 * t18435 * t1963 * t4541 - t18838 * t1940 * t7091 - 6.0_f64 * t18875 * t2403 * t27368 - 2.0_f64 * t1940 * t27368 * t4537 + 2.0_f64 * t1940 * t6079 * t93404 - 6.0_f64 * t2403 * t27368 * t27375 + 3.0_f64 * t2403 * t29705 * t775 - 3.0_f64 * t2403 * t7091 * t77425 - 6.0_f64 * t2403 * t7091 * t77441 + 6.0_f64 * t4541 * t5966 * t7087 + 6.0_f64 * t29907 * t50080;
    t107867
}
