//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 782/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk782<F: Float>(t1955: F, t8471: F, t1957: F, t233: F, t1954: F, t209: F, t2452: F) -> (F, F, F, F) {
    let t8472 = t1955 * t8471;
    let t8473 = t1957 * t233;
    let t8476 = t1954 * t209;
    let t8477 = t8476 * t2452;
    (t8472, t8473, t8476, t8477)
}
