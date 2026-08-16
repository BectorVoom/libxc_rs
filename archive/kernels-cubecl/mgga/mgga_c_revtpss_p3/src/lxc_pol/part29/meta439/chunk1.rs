//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1650/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1650<F: Float>(t1450: F, t3889: F, t2242: F, t607: F, t640: F, t644: F, t77: F, t2315: F, t84: F, t2251: F, t603: F, t2259: F) -> (F, F, F, F, F, F) {
    let t25089 = t1450 * t3889;
    let t25102 = t2242 * t607;
    let t25110 = t77 * t640 * t644;
    let t25113 = t84 * t2315;
    let t25114 = t77 * t25113;
    let t25117 = t603 * t2251;
    let t25120 = t603 * t2259;
    (t25089, t25102, t25110, t25114, t25117, t25120)
}
