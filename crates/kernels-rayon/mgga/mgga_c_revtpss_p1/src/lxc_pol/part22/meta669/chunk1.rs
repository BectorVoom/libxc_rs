//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2635/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2635(t1225: f64, t18281: f64, t1012: f64, t1010: f64, t5843: f64, t5378: f64, t5381: f64, t21040: f64, t3629: f64, t3626: f64, t12840: f64, t20795: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21209 = t1225 * t18281;
    let t21210 = t1012 * t21209;
    let t21213 = t5843 * t1010;
    let t21216 = t5381 * t5378;
    let t21218 = t21040 * t3629;
    let t21219 = t3626 * t21218;
    let t21222 = t20795 * t12840;
    (t21209, t21210, t21213, t21216, t21218, t21219, t21222)
}
