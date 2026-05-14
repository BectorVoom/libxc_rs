//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 705/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk705<F: Float>(t30: F, t265: F, t393: F, t1518: F, t2163: F, t7855: F, t1469: F, t2129: F, t45: F, t7794: F, t1479: F, t343: F, t136: F, t1785: F, t2138: F, t1802: F, t2137: F, t467: F, t1782: F, t1791: F, t1797: F, t1808: F, t464: F, t484: F, t7606: F, t7607: F, t7613: F, t7618: F, t7622: F, t7624: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t120 = rho0 <= dens_threshold || t31;
    let t394 = t265 < t393;
    let t8158 = t2163 * t1518;
    let t8161 = piecewise3(t394, 0.0, t7855);
    let t8166 = piecewise3(t120, t7794, t2129 * t1469 / 2.0 + t8161 * t45 / 2.0);
    let t8171 = t1479 * t343;
    let t8172 = t8171 * t136;
    let t8177 = t1785 * t2138;
    let t8184 = t2137 * t1802;
    let t8185 = t467 * t8184;
    let t8190 = -t8172 * t464 / 36.0 + t7606 - t7607 * t1782 / 288.0 + 0.42874018118069736972e-3 * t8177 * t484 - 0.42874018118069736972e-3 * t7613 * t1791 + 0.42874018118069736972e-3 * t7618 * t1797 - 0.22866142996303859718e-2 * t8185 * t484 + t7622 - 0.28582678745379824648e-3 * t7624 * t1808;
    (t8158, t8161, t8166, t8171, t8172, t8177, t8184, t8185, t8190)
}
