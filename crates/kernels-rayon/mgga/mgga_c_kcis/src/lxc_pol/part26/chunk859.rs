//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 859/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk859(t12844: f64, t6172: f64, t4439: f64, t1607: f64, t5713: f64, t110: f64, t2105: f64, t1599: f64, t25: f64, t6184: f64, t4429: f64, t6141: f64) -> (f64, f64, f64, f64, f64) {
    let t18091 = t12844 * t6172;
    let t18093 = t4439 * t18091 / 864.0_f64;
    let t18128 = t5713 * t1607;
    let t18141 = t110 * t2105;
    let t18142 = t1599 * t18141;
    let t18146 = t25 * t6184;
    let t18148 = t1599 * t18146 / 288.0_f64;
    let t18152 = t6141 * t4429 / 108.0_f64;
    (t18093, t18128, t18142, t18148, t18152)
}
