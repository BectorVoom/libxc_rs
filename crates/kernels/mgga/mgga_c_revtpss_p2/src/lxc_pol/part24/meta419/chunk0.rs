//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1367/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1367<F: Float>(t43813: F, t241: F, t281: F, t414: F, t39484: F, t403: F, t409: F, t13099: F, t159: F, t406: F, t3382: F, t3431: F, t408: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43814 = F::cast_from(0.31310740740740740741e1_f64) * t43813;
    let t43816 = t281 * t241 * t414;
    let t43817 = F::cast_from(0.13490888888888888889e1_f64) * t43816;
    let t43821 = F::new(1.0) / t409 / t39484 / t403 / F::new(96.0);
    let t43860 = t159 * t13099;
    let t43881 = F::new(280.0) / F::new(81.0) * t43813;
    let t43946 = F::powf(t406, -F::new(0.25e1));
    let t43995 = F::cast_from(0.96141975308641975307e-1_f64) * t43813;
    let t44017 = t408 / t3431 / t3382;
    (t43814, t43816, t43817, t43821, t43860, t43881, t43946, t43995, t44017)
}
