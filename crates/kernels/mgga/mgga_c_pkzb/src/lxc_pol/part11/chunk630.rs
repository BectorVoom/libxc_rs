//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 630/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk630<F: Float>(t218: F, t219: F, t3542: F, t208: F, t3515: F, t1870: F, t1881: F, t2730: F, t2772: F, t3517: F, t3529: F, t3533: F, t3537: F, t3539: F) -> (F, F, F, F) {
    let t3544 = t218 * t219 * t3542;
    let t3546 = t208 * t3515;
    let t3548 = t218 * t219 * t3546;
    let t3550 = -F::new(0.9494625e0) * t3529 + F::new(0.1898925e1) * t3533 + t1870 - F::new(0.59793333333333333334e0) * t2730 + F::new(0.8969e0) * t3517 + F::new(0.15358125e0) * t3537 + F::new(0.3071625e0) * t3539 + t1881 - F::new(0.32862666666666666666e0) * t2772 + F::new(0.24647e0) * t3544 + F::new(0.24647e0) * t3548;
    (t3544, t3546, t3548, t3550)
}
