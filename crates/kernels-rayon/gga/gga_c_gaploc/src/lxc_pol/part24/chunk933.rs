//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 933/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk933(t10144: f64, t2343: f64, t2268: f64, t10115: f64, t10118: f64, t10119: f64, t10124: f64, t10127: f64, t10131: f64, t10134: f64, t10137: f64, t10139: f64, t10143: f64, t1063: f64, t9072: f64, t9077: f64, t9085: f64) -> (f64, f64) {
    let t10145 = t2343 * t10144;
    let t10147 = 0.56910013271352299198e-1_f64 * t2268 * t10145;
    let t10148 = -t9072 + t9077 + t9085 + t10115 + t10118 - 0.28455006635676149599e-1_f64 * t1063 * t10119 + 0.28455006635676149599e-1_f64 * t1063 * t10124 + 0.28455006635676149599e-1_f64 * t2268 * t10127 - t10131 - t10134 - t10137 + t10139 + t10143 + t10147;
    (t10145, t10148)
}
