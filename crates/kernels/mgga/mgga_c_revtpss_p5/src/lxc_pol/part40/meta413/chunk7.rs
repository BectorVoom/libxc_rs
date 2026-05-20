//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1501/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1501<F: Float>(t117845: F, t117889: F, t118039: F, t118083: F, t2204: F, t5808: F, t1921: F, t8330: F, t1913: F, t8349: F, t31512: F, t571: F) -> (F, F, F, F, F) {
    let t118085 = t117845 + t117889 + t118039 + t118083;
    let t118089 = F::new(2.0) * t2204 * t5808;
    let t118091 = F::new(2.0) * t8330 * t1921;
    let t118094 = F::new(2.0) * t1913 * t8349;
    let t118099 = F::new(2.0) * t571 * t31512;
    (t118085, t118089, t118091, t118094, t118099)
}
