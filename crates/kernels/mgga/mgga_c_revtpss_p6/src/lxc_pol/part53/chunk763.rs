//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 763/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk763<F: Float>(t572: F, t7953: F, t1918: F, t2040: F, t573: F, t7944: F, t7949: F, t7952: F, t1469: F, t1479: F, t61: F, t6971: F, t7571: F) -> (F, F, F) {
    let t7955 = F::new(3.0) * t572 * t7953;
    let t7956 = F::new(3.0) * t1918 * t2040 + t573 * t7944 + t7949 + t7952 + t7955;
    let t8142 = -F::new(8.0) / F::new(3.0) * t1479 * t61 - F::new(5.0) / F::new(6.0) * t7571 * t1469 + t6971;
    (t7955, t7956, t8142)
}
