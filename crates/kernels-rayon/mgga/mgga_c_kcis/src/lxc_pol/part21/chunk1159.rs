//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1159/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1159(t28123: f64, t4547: f64, t5302: f64, t1262: f64, t1856: f64, t26996: f64, t5329: f64) -> (f64, f64, f64, f64) {
    let t28124 = t28123 * t4547;
    let t28125 = t5302 * t28124;
    let t28130 = t1856 * t1262;
    let t28131 = t26996 * t28130;
    let t28132 = t5329 * t28131;
    (t28124, t28125, t28131, t28132)
}
