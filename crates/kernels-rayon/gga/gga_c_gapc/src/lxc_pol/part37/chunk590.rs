//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 590/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk590(t3391: f64, t3392: f64, t3355: f64, t3358: f64, t3361: f64, t3365: f64, t3369: f64, t3372: f64, t3376: f64, t3380: f64, t3385: f64, t3389: f64) -> (f64, f64) {
    let t3393 = t3391 * t3392;
    let t3395 = 0.2318836277704281739e-4_f64 * t3355 + 0.19323635647535681159e-6_f64 * t3358 - 0.343574241813184411e-6_f64 * t3361 - 0.42205124476153752644e-7_f64 * t3365 - 0.42205124476153752644e-7_f64 * t3369 + 0.30950424615846085272e-6_f64 * t3372 + 0.14068374825384584215e-7_f64 * t3376 - 0.13900948042322754167e-2_f64 * t3380 + 0.6081664768516204948e-3_f64 * t3385 - 0.50602213541666666669e-5_f64 * t3389 - 0.50602213541666666669e-5_f64 * t3393;
    (t3393, t3395)
}
