//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 429/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk429<F: Float>(t2130: F, t4: F, t1487: F, t754: F, t110: F, t656: F, t1741: F, t1762: F, t758: F, t10: F, t107: F, t183: F, t1931: F, t2113: F, t2117: F, t2123: F, t2125: F, t2129: F, t266: F, t305: F, t306: F, t677: F, t749: F, t753: F, t755: F, t759: F, t79: F) -> (F,) {
    let t2131 = t4 * t2130;
    let t2134 = t754 * t1487;
    let t2137 = t110 * t656;
    let t2138 = t2137 * t1741;
    let t2141 = t758 * t1762;
    let t2153 = 0.58998125e-2 * t2113 * t306 - 0.2359925e-1 * t2117 * t755 - 0.11799625e-1 * t749 * t759 + 0.19666041666666666667e-2 * t2123 * t2125 + 0.2359925e-1 * t2129 * t2131 + 0.15732833333333333333e-1 * t753 * t2134 + 0.11799625e-1 * t305 * t2138 - 0.58998125e-2 * t305 * t2141 + 0.47803703703703703703e-2 * t107 * t79 * t266 - 0.28682222222222222222e-1 * t107 * t10 * t677 - 0.21511666666666666667e-1 * t107 * t183 * t1931;
    (t2153,)
}
