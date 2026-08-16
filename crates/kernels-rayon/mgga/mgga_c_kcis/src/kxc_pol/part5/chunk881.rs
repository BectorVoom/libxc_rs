//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 881/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk881(t4134: f64, t6922: f64, t572: f64, t571: f64, t1494: f64, t7202: f64, t584: f64, t6927: f64, t583: f64, t4286: f64, t552: f64, t7192: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7309 = t4134 * t6922;
    let t7310 = t572 * t7309;
    let t7311 = t571 * t7310;
    let t7313 = t1494 * t7202;
    let t7314 = t572 * t7313;
    let t7315 = t571 * t7314;
    let t7317 = t584 * t6927;
    let t7318 = t583 * t7317;
    let t7319 = t4286 * t7318;
    let t7321 = t7192 * t552;
    (t7309, t7310, t7311, t7313, t7314, t7315, t7318, t7319, t7321)
}
