//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1045/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1045(t11068: f64, t2617: f64, t7803: f64, t41281: f64, t41283: f64, t41286: f64, t41290: f64, t41293: f64, t41305: f64, t41307: f64, t13016: f64, t8478: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43881 = t7803 * t11068 * t2617;
    let t43882 = 0.76685851907841499353e0_f64 * t43881;
    let t43883 = 0.29792074959875355558e-1_f64 * t41281;
    let t43884 = 0.29792074959875355558e-1_f64 * t41283;
    let t43885 = 0.29792074959875355558e-1_f64 * t41286;
    let t43886 = 0.29792074959875355558e-1_f64 * t41290;
    let t43887 = 0.59584149919750711116e-1_f64 * t41293;
    let t43890 = 0.59584149919750711116e-1_f64 * t41305;
    let t43891 = 0.89376224879626066674e-1_f64 * t41307;
    let t43895 = 0.10725146985555128001e1_f64 * t8478 * t13016;
    (t43882, t43883, t43884, t43885, t43886, t43887, t43890, t43891, t43895)
}
