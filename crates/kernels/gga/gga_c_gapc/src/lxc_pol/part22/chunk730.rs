//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 730/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk730<F: Float>(t1510: F, t493: F, t2928: F, t1273: F, t991: F, t1007: F, t1484: F, t1492: F, t8406: F, t8409: F, t8413: F, t8417: F, t8420: F, t8423: F, t8428: F) -> (F, F, F, F, F) {
    let t8430 = t493 * t1510;
    let t8431 = t2928 * t8430;
    let t8433 = t1273 * t991;
    let t8435 = t1484 * t1007;
    let t8437 = t1492 * t1007;
    let t8439 = -F::new(0.27517776890953574544e-3) * t8406 + F::new(0.43449121406768801912e-4) * t8409 + F::new(0.20855578275249024918e-2) * t8413 + F::new(0.6951859425083008306e-4) * t8417 - F::new(0.20245571104589666024e-4) * t8420 + F::new(0.97853593672183385784e-4) * t8423 + F::new(0.12360406057797588768e-3) * t8428 + F::new(0.60736713313768998074e-4) * t8431 + F::new(0.46345729500553388707e-2) * t8433 - F::new(0.7724288250092231451e-3) * t8435 + F::new(0.27517776890953574544e-3) * t8437;
    (t8431, t8433, t8435, t8437, t8439)
}
