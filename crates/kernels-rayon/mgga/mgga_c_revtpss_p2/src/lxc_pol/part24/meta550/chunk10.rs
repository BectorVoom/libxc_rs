//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1636/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1636(t39989: f64, t40115: f64, t40131: f64, t40137: f64, t87655: f64, t87658: f64, t87660: f64, t87661: f64, t87662: f64, t87663: f64, t87666: f64, t87667: f64, t87668: f64, t87669: f64) -> f64 {
    let t87952 = t87655 - t40115 + t87658 + t87660 - t40131 - t40137 + t87661 + t87662 + t87663 + t87666 + t87667 - t39989 - t87668 - t87669;
    t87952
}
