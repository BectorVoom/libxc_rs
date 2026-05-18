//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 978/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk978<F: Float>(t13272: F, t7565: F, t38: F, t8142: F, t2247: F, t2123: F, t26749: F, t26755: F, t28105: F, t28109: F, t28112: F, t28116: F, t28119: F, t28133: F, t28141: F, t6960: F, t6963: F, t7566: F, t7576: F, t7579: F, t7706: F, t7709: F, t8144: F) -> (F, F) {
    let t29388 = t13272 * t7565;
    let t29411 = t38 * t8142;
    let t29412 = t2247 * t29411;
    let t29419 = F::new(5.0) / F::new(6.0) * t29388 * t6960 + t28141 * t2123 / F::new(3.0) + F::new(5.0) / F::new(6.0) * t26749 * t7706 + F::new(5.0) / F::new(6.0) * t26755 * t7706 + F::new(5.0) / F::new(6.0) * t7566 * t28105 + F::new(5.0) / F::new(6.0) * t7566 * t28109 + t28112 * t2123 / F::new(3.0) + t28116 * t2123 / F::new(3.0) + t28119 * t2123 / F::new(3.0) + t7709 * t7576 / F::new(3.0) + t7709 * t7579 / F::new(3.0) + F::new(5.0) / F::new(6.0) * t29412 * t6960 + t6963 * t8144 / F::new(3.0) + F::new(5.0) / F::new(6.0) * t7566 * t28133;
    (t29411, t29419)
}
