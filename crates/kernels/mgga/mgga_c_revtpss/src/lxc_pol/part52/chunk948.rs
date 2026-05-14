//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 948/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk948<F: Float>(t1925: F, t36: F, t606: F, t8442: F, t624: F, t8435: F, t2247: F) -> (F, F, F, F) {
    let t32591 = t1925 * t36;
    let t32592 = t32591 * t606;
    let t32593 = t8442 * t32592;
    let t32596 = t8435 * t624;
    let t32597 = t2247 * t32596;
    (t32591, t32593, t32596, t32597)
}
