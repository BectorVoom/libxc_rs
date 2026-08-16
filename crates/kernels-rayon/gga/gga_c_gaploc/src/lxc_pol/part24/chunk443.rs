//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 443/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk443(t1741: f64, t2137: f64, t1762: f64, t758: f64, t10: f64, t107: f64, t183: f64, t1931: f64, t2113: f64, t2117: f64, t2123: f64, t2125: f64, t2129: f64, t2131: f64, t2134: f64, t266: f64, t305: f64, t306: f64, t677: f64, t749: f64, t753: f64, t755: f64, t759: f64, t79: f64) -> f64 {
    let t2138 = t2137 * t1741;
    let t2141 = t758 * t1762;
    let t2153 = 0.58998125e-2_f64 * t2113 * t306 - 0.2359925e-1_f64 * t2117 * t755 - 0.11799625e-1_f64 * t749 * t759 + 0.19666041666666666667e-2_f64 * t2123 * t2125 + 0.2359925e-1_f64 * t2129 * t2131 + 0.15732833333333333333e-1_f64 * t753 * t2134 + 0.11799625e-1_f64 * t305 * t2138 - 0.58998125e-2_f64 * t305 * t2141 + 0.47803703703703703703e-2_f64 * t107 * t79 * t266 - 0.28682222222222222222e-1_f64 * t107 * t10 * t677 - 0.21511666666666666667e-1_f64 * t107 * t183 * t1931;
    t2153
}
