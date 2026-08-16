//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1147/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1147(t1445: f64, t38974: f64, t813: f64, t935: f64, t44114: f64, t44118: f64, t44120: f64, t44124: f64, t44128: f64, t44131: f64, t47527: f64, t47531: f64, t47535: f64, t47537: f64, t47540: f64) -> f64 {
    let t47544 = t813 * t1445 * t38974 * t935;
    let t47547 = -0.44688112439813033337e-1_f64 * t44114 - t44118 + 0.25561950635947166451e0_f64 * t44120 + 0.42603251059911944084e-1_f64 * t44124 - 0.69017266717057349418e1_f64 * t47527 - 0.69017266717057349418e1_f64 * t47531 - 0.69017266717057349418e1_f64 * t47535 + 0.11502877786176224903e2_f64 * t47537 + 0.11502877786176224903e2_f64 * t47540 - 0.46011511144704899612e1_f64 * t47544 - 0.42603251059911944084e-1_f64 * t44128 - t44131;
    t47547
}
