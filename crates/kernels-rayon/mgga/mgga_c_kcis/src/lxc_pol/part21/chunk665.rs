//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 665/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk665(t413: f64, t1851: f64, t3530: f64, t1262: f64, t5329: f64, t5272: f64) -> (f64, f64, f64, f64) {
    let t418 = 0.0_f64 < t413;
    let t5330 = t3530 * t1851;
    let t5331 = t5330 * t1262;
    let t5332 = t5329 * t5331;
    let t5336 = piecewise3(t418, t5272, -t5272);
    (t5330, t5331, t5332, t5336)
}
