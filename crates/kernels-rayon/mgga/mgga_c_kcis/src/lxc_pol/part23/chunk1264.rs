//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1264/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1264(t18210: f64, t2237: f64, t28534: f64, t1386: f64, t16831: f64, t4121: f64, t491: f64, t556: f64, t15888: f64, t4160: f64, t16638: f64, t94425: f64) -> (f64, f64, f64, f64) {
    let t98652 = 0.46336805555555555556e-3_f64 * t2237 * t18210 * t28534;
    let t98653 = t16831 * t1386;
    let t98661 = t4121 * t491 * t556;
    let t98663 = t4160 * t98661 * t15888;
    let t98666 = t4160 * t94425 * t16638;
    (t98652, t98653, t98663, t98666)
}
