//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 904/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk904<F: Float>(t12839: F, t1469: F, t20795: F, t3626: F, t6638: F, t17304: F, t17340: F, t17342: F, t17438: F, t1791: F, t20817: F, t20843: F, t20847: F, t20851: F, t20917: F, t20927: F, t20966: F, t21177: F, t5331: F, t5340: F, t6611: F) -> (F,) {
    let t24567 = t12839 * t1469;
    let t24568 = t20795 * t24567;
    let t24569 = t3626 * t24568;
    let t24572 = t20795 * t6638;
    let t24573 = t3626 * t24572;
    let t24587 = 0.42874018118069736972e-3 * t20817 - 0.42874018118069736972e-3 * t20843 + 0.85748036236139473944e-3 * t20847 + 0.14291339372689912324e-3 * t17304 - 0.85748036236139473944e-3 * t5340 * t24569 + 0.42874018118069736972e-3 * t5331 * t24573 + 0.85748036236139473944e-3 * t20917 + 0.7622047665434619906e-3 * t17340 - 0.14291339372689912324e-3 * t17342 - 0.21722835846488666732e-1 * t21177 * t1791 - 0.68598428988911579154e-2 * t17438 * t6611 - 0.85748036236139473944e-3 * t20927 + 11.0 / 108.0 * t20966 - 0.64311027177104605458e-3 * t20851 * t1791;
    (t24587,)
}
