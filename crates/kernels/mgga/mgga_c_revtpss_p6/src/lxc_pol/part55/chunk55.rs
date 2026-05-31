//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 55/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk55<F: Float>(t128: F, t131: F, t134: F, t141: F) -> (F, F, F, F, F) {
    let t164 = F::cast_from(1.0_f64) + F::cast_from(0.5137e-1_f64) * t128;
    let t169 = F::cast_from(0.705945e1_f64) * t131 + F::cast_from(0.1549425e1_f64) * t128 + F::cast_from(0.420775e0_f64) * t134 + F::cast_from(0.1562925e0_f64) * t141;
    let t172 = F::cast_from(1.0_f64) + F::cast_from(0.32163958997385070134e2_f64) / t169;
    let t173 = F::ln(t172);
    let t177 = F::cast_from(1.0_f64) + F::cast_from(0.278125e-1_f64) * t128;
    (t164, t169, t172, t173, t177)
}
