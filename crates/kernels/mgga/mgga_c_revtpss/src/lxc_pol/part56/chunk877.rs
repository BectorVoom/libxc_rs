//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 877/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk877<F: Float>(t33612: F, t8621: F, t1469: F, t32143: F, t1493: F, t84: F, t1501: F, t8453: F, t7741: F, t8692: F, t4248: F, t8460: F, t7889: F, t7742: F, t8634: F, t4147: F, t7933: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t33613 = t8621 * t33612;
    let t33617 = t8621 * t32143 * t1469;
    let t33624 = t84 * t1493;
    let t33625 = t8621 * t33624;
    let t33639 = t1501 * t8453;
    let t33640 = 2.0 * t33639;
    let t33642 = 4.0 * t8692 * t7741;
    let t33643 = t4248 * t8460;
    let t33644 = 2.0 * t33643;
    let t33645 = t7889 * t8460;
    let t33646 = 2.0 * t33645;
    let t33650 = 4.0 * t8634 * t7742;
    let t33651 = t4147 * t7933;
    (t33613, t33617, t33624, t33625, t33639, t33640, t33642, t33644, t33646, t33650, t33651)
}
