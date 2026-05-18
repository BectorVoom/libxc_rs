//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 589/1444 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk589<F: Float>(t3391: F, t3392: F, t3355: F, t3358: F, t3361: F, t3365: F, t3369: F, t3372: F, t3376: F, t3380: F, t3385: F, t3389: F) -> (F, F) {
    let t3393 = t3391 * t3392;
    let t3395 = F::new(0.2318836277704281739e-4) * t3355 + F::new(0.19323635647535681159e-6) * t3358 - F::new(0.343574241813184411e-6) * t3361 - F::new(0.42205124476153752644e-7) * t3365 - F::new(0.42205124476153752644e-7) * t3369 + F::new(0.30950424615846085272e-6) * t3372 + F::new(0.14068374825384584215e-7) * t3376 - F::new(0.13900948042322754167e-2) * t3380 + F::new(0.6081664768516204948e-3) * t3385 - F::new(0.50602213541666666669e-5) * t3389 - F::new(0.50602213541666666669e-5) * t3393;
    (t3393, t3395)
}
