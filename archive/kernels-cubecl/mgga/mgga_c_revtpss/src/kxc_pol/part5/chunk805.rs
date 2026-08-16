//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 805/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk805<F: Float>(t1892: F, t212: F, t1358: F, t689: F, t1893: F, t786: F, t1364: F, t1889: F, t3989: F, t1882: F, t550: F, t543: F) -> (F, F, F, F, F, F, F, F) {
    let t5599 = t212 * t1892;
    let t5600 = t5599 * t1358;
    let t5601 = t689 * t5600;
    let t5603 = t786 * t1893;
    let t5604 = t5603 * t1364;
    let t5606 = t3989 * t1889;
    let t5608 = t550 * t1882;
    let t5609 = t5608 * t543;
    (t5599, t5600, t5601, t5603, t5604, t5606, t5608, t5609)
}
