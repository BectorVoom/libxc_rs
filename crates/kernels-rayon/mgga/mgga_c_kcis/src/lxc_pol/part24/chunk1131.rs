//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1131/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1131(t2593: f64, t9053: f64, t2150: f64, t755: f64, t8750: f64, t2484: f64, t26550: f64, t26527: f64, t9042: f64, t26553: f64, t815: f64, t808: f64, t9046: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91811 = t2593 * t9053;
    let t91814 = t755 * t2150 * t8750;
    let t91816 = t2484 * t26550;
    let t91818 = t9042 * t26527;
    let t91820 = t815 * t26553;
    let t91822 = t808 * t9046;
    (t91811, t91814, t91816, t91818, t91820, t91822)
}
