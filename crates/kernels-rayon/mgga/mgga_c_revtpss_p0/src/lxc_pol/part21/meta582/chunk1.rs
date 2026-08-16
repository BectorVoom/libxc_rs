//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2293/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2293(t1294: f64, t5245: f64, t1277: f64, t1774: f64, t3737: f64, t3738: f64, t460: f64, t5412: f64, t17306: f64, t487: f64, t1269: f64, t5219: f64) -> (f64, f64, f64, f64, f64) {
    let t18042 = t5245 * t1294;
    let t18043 = t1277 * t18042;
    let t18047 = t3737 * t1774 * t3738;
    let t18054 = t460 * t5412;
    let t18059 = t17306 * t487;
    let t18062 = t5219 * t1269;
    (t18043, t18047, t18054, t18059, t18062)
}
