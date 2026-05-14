//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 444/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk444<F: Float>(t1781: F, t209: F, t184: F, t221: F, t1675: F, t1677: F, t1682: F, t1685: F, t1728: F, t1732: F, t1737: F, t1739: F, t1742: F, t1752: F, t1777: F, t1780: F) -> (F, F, F, F) {
    let t1782 = t1781 * t209;
    let t1783 = t1782 * t184;
    let t1785 = 4.0 / 15.0 * t1783 * t221;
    let t1786 = -t1675 + t1677 + t1682 - t1685 - t1728 + t1732 + t1737 - t1739 - t1742 + t1752 + t1777 - t1780 + t1785;
    (t1782, t1783, t1785, t1786)
}
