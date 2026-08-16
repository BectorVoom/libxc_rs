//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 525/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk525(t482: f64, t5586: f64, t1911: f64, t45: f64, t1919: f64, t3918: f64, t1578: f64, t3795: f64, t3881: f64, t3926: f64, t3933: f64, t5469: f64, t5472: f64, t5475: f64, t5479: f64, t5514: f64, t5516: f64, t5557: f64, t5559: f64, t5562: f64, t5565: f64, t5568: f64, t5571: f64) -> (f64, f64, f64, f64, f64) {
    let t5587 = t5586 * t482;
    let t5590 = t45 * t1911;
    let t5595 = t3918 * t1919;
    let t5596 = t5595 * t1578;
    let t5613 = -0.1294625e1_f64 * t5514 + 0.258925e1_f64 * t5516 + t3926 + 0.10064166666666666667e0_f64 * t3795 + 0.10064166666666666667e0_f64 * t5469 - 0.20128333333333333333e0_f64 * t5472 + 0.60385e0_f64 * t5475 + 0.60385e0_f64 * t5479 + 0.82524375e-1_f64 * t5557 + 0.16504875e0_f64 * t5559 + t3933 + 0.5519e-1_f64 * t3881 + 0.5519e-1_f64 * t5562 - 0.27595e-1_f64 * t5565 + 0.16557e0_f64 * t5568 + 0.16557e0_f64 * t5571;
    (t5587, t5590, t5595, t5596, t5613)
}
