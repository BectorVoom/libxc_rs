//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1073/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1073<F: Float>(t2052: F, t2089: F, t2108: F, t2127: F, t2163: F, t32415: F, t32417: F, t32419: F, t32421: F, t32576: F, t32580: F, t32822: F, t7357: F, t7374: F, t7378: F, t7474: F, t7537: F, t7584: F, t7586: F, t7683: F, t8764: F) -> F {
    let t33257 = -t2052 * t7683 - t2089 * t7584 + t2108 * t32822 - t2127 * t7474 - t2163 * t7357 - F::new(2.0) * t7374 * t7586 - F::new(2.0) * t7378 * t7586 + t7537 * t8764 - t32415 - t32417 - t32419 - t32421 - t32576 + t32580;
    t33257
}
