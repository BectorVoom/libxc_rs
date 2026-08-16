//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1065/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1065(t121072: f64, t25304: f64, t32217: f64, t8477: f64, t8705: f64, t9656: f64, t3999: f64, t8578: f64, t25880: f64, t676: f64, t7274: f64, t32705: f64) -> (f64, f64, f64, f64, f64) {
    let t121074 = 0.45699670022203476294e-2_f64 * t25304 * t32217 * t121072;
    let t121076 = t8477 * t8705 * t9656;
    let t121077 = t3999 * t8578;
    let t121086 = t25880 * t676 * t7274;
    let t121087 = t32705 * t121086;
    (t121074, t121076, t121077, t121086, t121087)
}
