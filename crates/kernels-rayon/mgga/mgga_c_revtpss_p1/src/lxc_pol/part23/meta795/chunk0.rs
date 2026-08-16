//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2616/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2616(t10760: f64, t18409: f64, t9794: f64, t10777: f64, t10779: f64, t5984: f64, t837: f64, t18414: f64, t40799: f64, t18418: f64, t18392: f64, t236: f64, t807: f64, t854: f64) -> (f64, f64, f64, f64, f64) {
    let t61981 = t10760 * t9794 * t18409;
    let t61985 = t10777 * t10779 * t5984 * t837;
    let t62012 = t40799 * t9794 * t18414;
    let t62015 = t10760 * t9794 * t18418;
    let t62021 = t807 * t236 * t854 * t18392;
    (t61981, t61985, t62012, t62015, t62021)
}
