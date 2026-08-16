//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 968/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk968(t22265: f64, t5661: f64, t11862: f64, t6905: f64, t167: f64, t2011: f64, t4171: f64, t4170: f64, t16771: f64, t1307: f64, t7392: f64, t12241: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22266 = t5661 * t22265;
    let t22268 = t11862 * t6905;
    let t22270 = t167 * t2011;
    let t22271 = t4171 * t22270;
    let t22272 = t4170 * t22271;
    let t22273 = t16771 * t22272;
    let t22275 = t7392 * t1307;
    let t22276 = t12241 * t22275;
    (t22266, t22268, t22271, t22273, t22275, t22276)
}
