//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 774/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk774<F: Float>(t1918: F, t2170: F, t573: F, t7949: F, t7952: F, t7955: F, t8245: F, t38: F, t73: F, t74: F, t84: F) -> (F, F, F, F) {
    let t8249 = F::new(3.0) * t1918 * t2170 + t573 * t8245 + t7949 + t7952 + t7955;
    let t8435 = t38 * t38;
    let t8440 = F::new(1.0) / t74 / t73;
    let t8441 = t84 * t84;
    (t8249, t8435, t8440, t8441)
}
