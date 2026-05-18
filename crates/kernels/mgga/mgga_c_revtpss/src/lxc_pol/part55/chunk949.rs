//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 949/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk949<F: Float>(t233: F, t28340: F, t1957: F, t2061: F, t231: F, t4423: F, t7076: F, t25317: F, t8006: F, t886: F, t4533: F, t7071: F) -> (F, F, F, F) {
    let t28399 = t233 * t28340;
    let t28400 = t1957 * t28399;
    let t28404 = t2061 * t4423 * t231;
    let t28405 = t7076 * t28404;
    let t28411 = t25317 * t8006 * t886;
    let t28417 = t2061 * t4533;
    let t28418 = t7071 * t28417;
    (t28400, t28405, t28411, t28418)
}
