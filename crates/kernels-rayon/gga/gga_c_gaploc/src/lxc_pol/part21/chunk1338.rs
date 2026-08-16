//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1338/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1338(t34862: f64, t2478: f64, t6583: f64, t8272: f64, t2890: f64, t7047: f64, t20670: f64, t20671: f64, t26922: f64, t21283: f64, t7035: f64, t10402: f64, t4811: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34863 = 0.59584149919750711116e-1_f64 * t34862;
    let t34865 = t6583 * t8272 * t2478;
    let t34866 = 0.38342925953920749676e0_f64 * t34865;
    let t34868 = t6583 * t2890 * t7047;
    let t34869 = 0.19171462976960374838e0_f64 * t34868;
    let t34873 = t20670 * t20671 * t26922;
    let t34874 = 0.85206502119823888168e-1_f64 * t34873;
    let t34876 = t21283 * t2890 * t7035;
    let t34877 = 0.38342925953920749676e0_f64 * t34876;
    let t34878 = t4811 * t10402;
    (t34863, t34866, t34869, t34874, t34877, t34878)
}
