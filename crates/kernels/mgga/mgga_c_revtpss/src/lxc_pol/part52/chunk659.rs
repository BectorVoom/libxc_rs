//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 659/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk659<F: Float>(t1364: F, t7496: F, t7250: F, t7257: F, t7260: F, t7267: F, t7253: F, t7265: F, t7272: F) -> (F, F, F, F, F, F) {
    let t7498 = 0.9757440539382783019e-2 * t7496 * t1364;
    let t7499 = 7.0 / 144.0 * t7250;
    let t7501 = 0.28582678745379824648e-4 * t7257;
    let t7502 = 0.50820002809285328225e-4 * t7260;
    let t7504 = 0.40015750243531754507e-2 * t7267;
    let t7506 = -t7499 - t7253 / 24.0 - t7501 + t7502 - 0.85748036236139473944e-3 * t7265 - t7504 - 0.34299214494455789578e-2 * t7272;
    (t7498, t7499, t7501, t7502, t7504, t7506)
}
