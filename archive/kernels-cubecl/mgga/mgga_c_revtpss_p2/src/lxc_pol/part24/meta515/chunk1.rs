//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1535/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1535<F: Float>(t11941: F, t127: F, t24032: F, t371: F, t15671: F, t20016: F, t1025: F, t24022: F, t1011: F, t15993: F, t23499: F, t11875: F, t11922: F, t24012: F) -> (F, F, F, F, F) {
    let t79742 = t11941 * t371 * t127 * t24032;
    let t79744 = t15671 * t20016;
    let t79758 = t1025 * t371 * t127 * t24022;
    let t79811 = t1011 * t15993 * t23499;
    let t79818 = t11875 * t11922 * t24012;
    (t79742, t79744, t79758, t79811, t79818)
}
