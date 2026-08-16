//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 844/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk844(t41667: f64, t1635: f64, t2064: f64, t4044: f64, t1550: f64, t7778: f64, t8377: f64, t1632: f64, t3928: f64, t2373: f64, t7561: f64, t40965: f64, t8620: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41668 = 0.43368970657079495312e-4_f64 * t41667;
    let t41716 = t4044 * t2064 * t1635;
    let t41717 = 0.95793933614910468512e0_f64 * t41716;
    let t41722 = t1550 * t7778 * t8377;
    let t41723 = 0.15965655602485078085e0_f64 * t41722;
    let t41725 = t3928 * t2064 * t1632;
    let t41726 = 0.47896966807455234256e0_f64 * t41725;
    let t41727 = t2373 * t7561;
    let t41735 = t8620 * t40965;
    (t41668, t41717, t41723, t41726, t41727, t41735)
}
