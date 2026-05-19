//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 325/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk325<F: Float>(t1404: F, t436: F, t468: F, t507: F, t1134: F, t1179: F, t1248: F, t1268: F, t1273: F, t1277: F, t1280: F, t174: F, t385: F, t426: F, t459: F, t466: F, t508: F, t526: F, t569: F) -> F {
    let t1405 = t436 * t1404;
    let t1408 = t468 * t507;
    let t1411 = -F::cast_from(0.3475929712541504153e-2_f64) * t1134 * t174 - F::cast_from(0.3475929712541504153e-2_f64) * t385 * t508 + F::cast_from(0.10427789137624512459e-2_f64) * t1268 * t174 + F::cast_from(0.20855578275249024918e-2_f64) * t426 * t508 + F::cast_from(0.46345729500553388707e-2_f64) * t1273 * t174 - t1179 + F::cast_from(0.3475929712541504153e-2_f64) * t1277 * t459 + F::cast_from(0.3475929712541504153e-2_f64) * t1280 * t569 - t1248 - F::cast_from(0.10427789137624512459e-2_f64) * t526 * t1405 - F::cast_from(0.6951859425083008306e-4_f64) * t466 * t1408;
    t1411
}
