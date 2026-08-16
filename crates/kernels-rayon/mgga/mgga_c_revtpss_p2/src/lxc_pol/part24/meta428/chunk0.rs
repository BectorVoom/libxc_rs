//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1378/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1378(t1209: f64, t17852: f64, t12627: f64, t3754: f64, t17948: f64, t3596: f64, t42859: f64, t460: f64, t3603: f64, t43351: f64, t1243: f64, t471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45659 = t1209 * t17852;
    let t45666 = t12627 * t3754;
    let t45738 = t1209 * t17948;
    let t45785 = t42859 * t3596;
    let t45786 = t460 * t45785;
    let t45787 = t43351 * t3603;
    let t45832 = t42859 * t1243;
    let t45833 = t460 * t45832;
    let t45834 = t43351 * t471;
    (t45659, t45666, t45738, t45786, t45787, t45833, t45834)
}
