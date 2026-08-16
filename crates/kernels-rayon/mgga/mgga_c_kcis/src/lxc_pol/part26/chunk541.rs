//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 541/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk541(t1495: f64, t5671: f64, t1468: f64, t1464: f64, t1489: f64, t2001: f64) -> (f64, f64, f64, f64) {
    let t5672 = t1495 * t5671;
    let t5673 = t1468 * t5672;
    let t5674 = t1464 * t5673;
    let t5676 = t2001 * t1489;
    (t5672, t5673, t5674, t5676)
}
