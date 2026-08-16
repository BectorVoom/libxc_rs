//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1401/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1401(t34644: f64, t34661: f64, t34663: f64, t34666: f64, t34669: f64, t34641: f64, t34647: f64, t34651: f64, t34654: f64, t34658: f64, t37020: f64, t34673: f64) -> (f64, f64) {
    let t37022 = 0.10793703140429833089e-5_f64 * t34644;
    let t37027 = 0.21720231316129303386e-4_f64 * t34661;
    let t37028 = 0.11372686522837130914e-5_f64 * t34663;
    let t37029 = 0.54924190264999682021e-4_f64 * t34666;
    let t37030 = 0.4637672555408563478e-4_f64 * t34669;
    let t37031 = -t37020 - 0.71958020936198887258e-7_f64 * t34641 + t37022 + 0.95956020918421216158e-7_f64 * t34647 + 0.98332751566569010434e-8_f64 * t34651 + 0.49166375783284505217e-8_f64 * t34654 - 0.65555167711046006956e-8_f64 * t34658 - t37027 - t37028 + t37029 + t37030;
    let t37032 = 0.69504740211613770836e-3_f64 * t34673;
    (t37031, t37032)
}
