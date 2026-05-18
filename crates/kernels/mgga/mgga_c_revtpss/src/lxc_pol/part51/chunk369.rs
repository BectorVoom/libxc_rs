//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 369/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk369<F: Float>(t1211: F, t1774: F, t1480: F, t344: F, t1225: F, t1469: F, t1012: F, t1770: F, t225: F, t480: F, t482: F, t372: F) -> (F, F, F, F, F, F, F, F) {
    let t1775 = t1211 * t1774;
    let t1778 = t1480 * t344;
    let t1781 = t1225 * t1469;
    let t1782 = t1012 * t1781;
    let t1785 = t1770 * t225;
    let t1786 = t1785 * t480;
    let t1789 = t482 * t1774;
    let t1790 = t372 * t1789;
    (t1775, t1778, t1781, t1782, t1785, t1786, t1789, t1790)
}
