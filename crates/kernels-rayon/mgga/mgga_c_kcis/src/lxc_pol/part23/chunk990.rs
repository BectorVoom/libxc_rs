//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 990/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk990(t4463: f64, t6177: f64, t6176: f64, t4426: f64, t6141: f64, t25: f64, t494: f64) -> (f64, f64, f64) {
    let t18200 = t6177 * t4463;
    let t18201 = t6176 * t18200;
    let t18205 = t6141 * t4426 / 324.0_f64;
    let t18210 = t25 * t494;
    (t18201, t18205, t18210)
}
