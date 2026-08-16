//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 222/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk222(t922: f64, t945: f64, t26: f64, t924: f64, t935: f64, t937: f64, t940: f64, t944: f64) -> (f64, f64, f64) {
    let t946 = t945 * t922;
    let t947 = t26 * t946;
    let t949 = 0.1898925e1_f64 * t935 - t937 - 0.29896666666666666667e0_f64 * t924 + 0.3071625e0_f64 * t940 - t944 - 0.82156666666666666667e-1_f64 * t947;
    (t946, t947, t949)
}
