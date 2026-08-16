//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1563/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1563<F: Float>(t43813: F, t241: F, t281: F, t414: F, t39484: F, t403: F, t409: F, t3391: F, t3399: F, t3390: F, t12288: F, t698: F) -> (F, F, F, F, F, F, F, F) {
    let t43814 = F::cast_from(0.31310740740740740741e1_f64) * t43813;
    let t43816 = t281 * t241 * t414;
    let t43817 = F::cast_from(0.13490888888888888889e1_f64) * t43816;
    let t43821 = F::cast_from(1.0_f64) / t409 / t39484 / t403 / F::cast_from(96.0_f64);
    let t43822 = t3391 * t3391;
    let t43823 = t43821 * t43822;
    let t43825 = t3399 * t3399;
    let t43826 = t3390 * t43825;
    let t43828 = t698 * t12288;
    (t43814, t43816, t43817, t43822, t43823, t43825, t43826, t43828)
}
