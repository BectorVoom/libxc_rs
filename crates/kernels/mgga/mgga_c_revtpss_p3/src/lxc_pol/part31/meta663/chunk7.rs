//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2252/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2252<F: Float>(t105818: F, t105822: F, t105826: F, t105830: F, t105834: F, t105837: F, t105839: F, t105841: F, t105843: F, t109278: F, t109282: F, t109288: F, t1461: F, t2040: F, t22556: F, t22568: F, t30171: F, t573: F, t5805: F, t6945: F, t7324: F, t7944: F, param_d: F) -> F {
    let t109289 = t109278 * t573 * param_d + F::cast_from(3.0_f64) * t1461 * t30171 + F::cast_from(6.0_f64) * t2040 * t22556 + F::cast_from(3.0_f64) * t2040 * t22568 + F::cast_from(6.0_f64) * t5805 * t7944 + F::cast_from(6.0_f64) * t6945 * t7324 + t105818 + t105822 + t105826 + t105830 + t105834 + t105837 + t105839 + t105841 + t105843 + t109282 + t109288;
    t109289
}
