//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1307/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1307<F: Float>(t231: F, t268: F, t2798: F, t39599: F, t10535: F, t281: F, t624: F, t836: F, t2722: F, t68: F, t10529: F, t2453: F) -> (F, F, F, F, F) {
    let t39668 = t2798 * t268 * t39599 * t231;
    let t39673 = t10535 * t281 * t624 * t836 * t231;
    let t39675 = t68 * t2722;
    let t39678 = t10535 * t281 * t39675 * t231;
    let t39680 = t2453 * t10529;
    (t39668, t39673, t39675, t39678, t39680)
}
