//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 346/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk346<F: Float>(t1169: F, t1744: F, t1173: F, t1717: F, t448: F, t1182: F, t1185: F, t1724: F, t1727: F, t1730: F) -> (F, F, F, F) {
    let t1745 = t1744 * t1169;
    let t1749 = -t1173 + 0.92708333333333333333e-2 * t1717;
    let t1750 = t1749 * t448;
    let t1756 = 0.258925e1 * t1724 - t1182 + 0.301925e0 * t1717 + 0.16504875e0 * t1727 - t1185 + 0.82785e-1 * t1730;
    (t1745, t1749, t1750, t1756)
}
