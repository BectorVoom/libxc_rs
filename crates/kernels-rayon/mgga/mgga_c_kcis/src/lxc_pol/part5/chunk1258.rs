//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1258/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1258(t5752: f64, t5781: f64, t5780: f64, t3738: f64, t7033: f64, t1394: f64, t18431: f64, t531: f64) -> (f64, f64, f64) {
    let t21014 = t5752 * t5781;
    let t21015 = t5780 * t21014;
    let t21017 = t3738 * t7033;
    let t21018 = t1394 * t21017;
    let t21020 = t531 * t18431;
    (t21015, t21018, t21020)
}
