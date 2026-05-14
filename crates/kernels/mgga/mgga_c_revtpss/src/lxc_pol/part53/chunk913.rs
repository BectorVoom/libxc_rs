//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 913/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk913<F: Float>(t25875: F, t8590: F, t1381: F, t31805: F, t555: F) -> (F, F, F) {
    let t32265 = t25875 * t8590;
    let t32266 = t32265 * t1381;
    let t32267 = 0.1859366460452550541e-4 * t32266;
    let t32268 = t31805 * t555;
    (t32265, t32267, t32268)
}
