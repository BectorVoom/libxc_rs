//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 713/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk713(t1574: f64, t2938: f64, t8369: f64, t8371: f64, t8374: f64, t8377: f64, t8382: f64, t8385: f64, t8388: f64, t8391: f64, t8397: f64, t8401: f64) -> f64 {
    let t8403 = t1574 * t2938;
    let t8405 = 0.3475929712541504153e-2_f64 * t8369 + 0.3475929712541504153e-2_f64 * t8371 + 0.20855578275249024918e-2_f64 * t8374 - 0.43449121406768801912e-5_f64 * t8377 + 0.257508459537449766e-6_f64 * t8382 - 0.772525378612349298e-5_f64 * t8385 - 0.10427789137624512459e-2_f64 * t8388 - 0.6951859425083008306e-4_f64 * t8391 + 0.73256006569213709438e-5_f64 * t8397 + 0.43449121406768801912e-4_f64 * t8401 - 0.27517776890953574544e-3_f64 * t8403;
    t8405
}
