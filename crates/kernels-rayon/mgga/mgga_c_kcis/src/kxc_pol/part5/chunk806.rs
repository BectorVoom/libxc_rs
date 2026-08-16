//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 806/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk806(t1035: f64, t6352: f64, t2918: f64, t4612: f64, t6328: f64, t6332: f64, t6336: f64, t261: f64, t1680: f64, t4685: f64, t1679: f64, t950: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6353 = t1035 * t6352;
    let t6360 = t2918 + 0.11872222222222222222e-1_f64 * t4612 - 0.11872222222222222222e-1_f64 * t6328 + 0.35616666666666666666e-1_f64 * t6332 - 0.17808333333333333333e-1_f64 * t6336;
    let t6362 = 0.62182e-1_f64 * t6360 * t261;
    let t6364 = 2.0_f64 * t4685 * t1680;
    let t6365 = t1679 * t1679;
    let t6366 = t6365 * t950;
    (t6353, t6360, t6362, t6364, t6365, t6366)
}
