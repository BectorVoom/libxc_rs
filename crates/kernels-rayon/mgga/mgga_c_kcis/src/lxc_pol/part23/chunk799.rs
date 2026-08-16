//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 799/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk799(t554: f64, t556: f64, t11782: f64, t577: f64, t1527: f64, t4121: f64, t4248: f64, t492: f64, t1591: f64, t4390: f64, t1370: f64, t4455: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12534 = 1.0_f64 / t556 / t554;
    let t12542 = t11782 * t577;
    let t12564 = t1527 * t4121;
    let t12568 = t4248 * t492;
    let t12581 = t4390 * t1591;
    let t12605 = t1370 * t4455;
    (t12534, t12542, t12564, t12568, t12581, t12605)
}
