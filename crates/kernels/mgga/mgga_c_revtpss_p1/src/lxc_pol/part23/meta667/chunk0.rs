//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2399/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2399<F: Float>(t11240: F, t3144: F, t42646: F, t11239: F, t989: F, t11629: F, t11874: F, t16048: F, t12046: F, t15905: F, t994: F, t1011: F, t1016: F, t2438: F) -> (F, F, F, F, F, F) {
    let t42648 = t11240 * t3144 * t42646;
    let t42668 = t989 * t11239;
    let t42669 = t42668 * t11629;
    let t42675 = t11874 * t16048;
    let t42690 = t994 * t12046 * t15905;
    let t42716 = t1011 * t2438 * t1016;
    (t42648, t42668, t42669, t42675, t42690, t42716)
}
