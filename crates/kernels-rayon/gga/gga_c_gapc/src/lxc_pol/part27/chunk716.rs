//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 716/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk716(t1510: f64, t493: f64, t2928: f64, t1273: f64, t991: f64, t1007: f64, t1484: f64, t1492: f64, t8406: f64, t8409: f64, t8413: f64, t8417: f64, t8420: f64, t8423: f64, t8428: f64) -> f64 {
    let t8430 = t493 * t1510;
    let t8431 = t2928 * t8430;
    let t8433 = t1273 * t991;
    let t8435 = t1484 * t1007;
    let t8437 = t1492 * t1007;
    let t8439 = -0.27517776890953574544e-3_f64 * t8406 + 0.43449121406768801912e-4_f64 * t8409 + 0.20855578275249024918e-2_f64 * t8413 + 0.6951859425083008306e-4_f64 * t8417 - 0.20245571104589666024e-4_f64 * t8420 + 0.97853593672183385784e-4_f64 * t8423 + 0.12360406057797588768e-3_f64 * t8428 + 0.60736713313768998074e-4_f64 * t8431 + 0.46345729500553388707e-2_f64 * t8433 - 0.7724288250092231451e-3_f64 * t8435 + 0.27517776890953574544e-3_f64 * t8437;
    t8439
}
