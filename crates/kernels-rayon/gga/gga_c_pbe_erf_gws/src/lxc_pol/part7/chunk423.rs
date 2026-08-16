//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 423/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk423(t1783: f64, t221: f64, t1675: f64, t1677: f64, t1682: f64, t1685: f64, t1728: f64, t1732: f64, t1737: f64, t1739: f64, t1742: f64, t1752: f64, t1777: f64, t1780: f64) -> (f64, f64) {
    let t1785 = 4.0_f64 / 15.0_f64 * t1783 * t221;
    let t1786 = -t1675 + t1677 + t1682 - t1685 - t1728 + t1732 + t1737 - t1739 - t1742 + t1752 + t1777 - t1780 + t1785;
    (t1785, t1786)
}
