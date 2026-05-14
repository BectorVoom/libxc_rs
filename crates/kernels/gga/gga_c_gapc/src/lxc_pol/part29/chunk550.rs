//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 550/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk550<F: Float>(t3391: F, t3392: F, t3355: F, t3358: F, t3361: F, t3365: F, t3369: F, t3372: F, t3376: F, t3380: F, t3385: F, t3389: F, t3103: F, t889: F, t2636: F, t787: F) -> (F, F, F) {
    let t3393 = t3391 * t3392;
    let t3395 = 0.2318836277704281739e-4 * t3355 + 0.19323635647535681159e-6 * t3358 - 0.343574241813184411e-6 * t3361 - 0.42205124476153752644e-7 * t3365 - 0.42205124476153752644e-7 * t3369 + 0.30950424615846085272e-6 * t3372 + 0.14068374825384584215e-7 * t3376 - 0.13900948042322754167e-2 * t3380 + 0.6081664768516204948e-3 * t3385 - 0.50602213541666666669e-5 * t3389 - 0.50602213541666666669e-5 * t3393;
    let t3396 = t889 * t3103;
    let t3397 = t2636 * t787;
    (t3395, t3396, t3397)
}
