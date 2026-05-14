//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 966/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk966<F: Float>(t1497: F, t8441: F, t7714: F, t8621: F, t1493: F, t84: F, t4248: F, t8460: F, t7889: F, t4147: F, t7933: F, t1559: F, t31756: F, t4364: F, t31755: F, t1544: F, t2747: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33612 = t8441 * t1497;
    let t33620 = t8441 * t7714;
    let t33621 = t8621 * t33620;
    let t33624 = t84 * t1493;
    let t33643 = t4248 * t8460;
    let t33644 = 2.0 * t33643;
    let t33645 = t7889 * t8460;
    let t33646 = 2.0 * t33645;
    let t33651 = t4147 * t7933;
    let t33674 = t4364 * t31756 * t1559;
    let t33675 = t31755 * t33674;
    let t33678 = t2747 * t31756 * t1544;
    (t33612, t33620, t33621, t33624, t33644, t33646, t33651, t33674, t33675, t33678)
}
