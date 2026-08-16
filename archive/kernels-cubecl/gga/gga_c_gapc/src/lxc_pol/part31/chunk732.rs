//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 732/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk732<F: Float>(t1574: F, t2938: F, t8369: F, t8371: F, t8374: F, t8377: F, t8382: F, t8385: F, t8388: F, t8391: F, t8397: F, t8401: F) -> (F, F) {
    let t8403 = t1574 * t2938;
    let t8405 = F::cast_from(0.3475929712541504153e-2_f64) * t8369 + F::cast_from(0.3475929712541504153e-2_f64) * t8371 + F::cast_from(0.20855578275249024918e-2_f64) * t8374 - F::cast_from(0.43449121406768801912e-5_f64) * t8377 + F::cast_from(0.257508459537449766e-6_f64) * t8382 - F::cast_from(0.772525378612349298e-5_f64) * t8385 - F::cast_from(0.10427789137624512459e-2_f64) * t8388 - F::cast_from(0.6951859425083008306e-4_f64) * t8391 + F::cast_from(0.73256006569213709438e-5_f64) * t8397 + F::cast_from(0.43449121406768801912e-4_f64) * t8401 - F::cast_from(0.27517776890953574544e-3_f64) * t8403;
    (t8403, t8405)
}
