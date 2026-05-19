//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 713/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk713<F: Float>(t1510: F, t493: F, t2928: F, t1273: F, t991: F, t1007: F, t1484: F, t1492: F, t8406: F, t8409: F, t8413: F, t8417: F, t8420: F, t8423: F, t8428: F) -> F {
    let t8430 = t493 * t1510;
    let t8431 = t2928 * t8430;
    let t8433 = t1273 * t991;
    let t8435 = t1484 * t1007;
    let t8437 = t1492 * t1007;
    let t8439 = -F::cast_from(0.27517776890953574544e-3_f64) * t8406 + F::cast_from(0.43449121406768801912e-4_f64) * t8409 + F::cast_from(0.20855578275249024918e-2_f64) * t8413 + F::cast_from(0.6951859425083008306e-4_f64) * t8417 - F::cast_from(0.20245571104589666024e-4_f64) * t8420 + F::cast_from(0.97853593672183385784e-4_f64) * t8423 + F::cast_from(0.12360406057797588768e-3_f64) * t8428 + F::cast_from(0.60736713313768998074e-4_f64) * t8431 + F::cast_from(0.46345729500553388707e-2_f64) * t8433 - F::cast_from(0.7724288250092231451e-3_f64) * t8435 + F::cast_from(0.27517776890953574544e-3_f64) * t8437;
    t8439
}
