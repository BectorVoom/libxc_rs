//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 363/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk363<F: Float>(t1120: F, t1715: F, t128: F, t1119: F, t422: F, t1118: F, t1132: F, t1139: F, t1145: F, t141: F, t1137: F, t1144: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t1716 = t1120 * t1715;
    let t1717 = t128 * t1716;
    let t1719 = -t1119 + F::new(0.17808333333333333333e-1) * t1717;
    let t1721 = F::new(0.621814e-1) * t1719 * t422;
    let t1723 = -t1118 / F::new(3.0) + t1717 / F::new(3.0);
    let t1724 = t1132 * t1723;
    let t1727 = t1139 * t1723;
    let t1729 = t1145 * t1715;
    let t1730 = t141 * t1729;
    let t1732 = F::new(0.1898925e1) * t1724 - t1137 + F::new(0.29896666666666666667e0) * t1717 + F::new(0.3071625e0) * t1727 - t1144 + F::new(0.82156666666666666667e-1) * t1730;
    (t1716, t1717, t1719, t1721, t1723, t1724, t1727, t1729, t1730, t1732)
}
