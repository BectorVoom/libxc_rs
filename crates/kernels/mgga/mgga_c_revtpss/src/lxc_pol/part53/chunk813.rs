//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 813/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk813<F: Float>(t4845: F, t7117: F, t1972: F, t4857: F, t1659: F, t7131: F, t25515: F, t4890: F, t3299: F, t1028: F, t1068: F, t1665: F, t1675: F, t25490: F, t25495: F, t25529: F, t25569: F, t25577: F, t4831: F, t4854: F, t4896: F, t7132: F) -> (F, F) {
    let t27471 = t7117 * t4845;
    let t27479 = t4857 * t1972;
    let t27489 = t1659 * t7131;
    let t27492 = t25515 * t4890;
    let t27493 = t3299 * t27492;
    let t27496 = -0.28582678745379824648e-3 * t27471 + 0.22866142996303859718e-2 * t25495 * t1665 - 0.42874018118069736972e-3 * t25490 * t1665 - 0.42874018118069736972e-3 * t7117 * t4854 - 0.42874018118069736972e-3 * t27479 * t1028 + 0.28582678745379824648e-3 * t25529 - 0.15244095330869239812e-2 * t25577 * t1675 + 0.28582678745379824648e-3 * t7132 * t4831 + 0.28582678745379824648e-3 * t25569 * t1675 + 0.28582678745379824648e-3 * t27489 * t1068 + 0.85748036236139473944e-3 * t27493 * t4896;
    (t27492, t27496)
}
