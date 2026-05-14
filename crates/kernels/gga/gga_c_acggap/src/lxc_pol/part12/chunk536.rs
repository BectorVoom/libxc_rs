//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 536/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk536<F: Float>(t1264: F, t322: F, t449: F, t316: F, t868: F, t872: F, t463: F, t879: F, t180: F, t848: F, t464: F, t310: F, t441: F, t852: F, t880: F, t851: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3882 = t322 * t1264;
    let t3883 = t449 * t3882;
    let t3884 = t316 * t3883;
    let t3886 = t868 * t872;
    let t3888 = t879 * t463;
    let t3889 = t449 * t3888;
    let t3890 = t316 * t3889;
    let t3892 = t848 * t180;
    let t3893 = t3892 * t464;
    let t3896 = t310 * t441;
    let t3897 = t3896 * t464;
    let t3900 = 0.19756347548806534796e1 * t852 * t880;
    let t3901 = t851 * t441;
    (t3883, t3884, t3886, t3889, t3890, t3892, t3893, t3896, t3897, t3900, t3901)
}
