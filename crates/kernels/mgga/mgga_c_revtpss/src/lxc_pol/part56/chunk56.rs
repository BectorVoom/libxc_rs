//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 56/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk56<F: Float>(t128: F, t131: F, t134: F, t141: F) -> (F, F, F, F, F) {
    let t164 = 1.0 + 0.5137e-1 * t128;
    let t169 = 0.705945e1 * t131 + 0.1549425e1 * t128 + 0.420775e0 * t134 + 0.1562925e0 * t141;
    let t172 = 1.0 + 0.32163958997385070134e2 / t169;
    let t173 = f64::ln(t172);
    let t177 = 1.0 + 0.278125e-1 * t128;
    (t164, t169, t172, t173, t177)
}
