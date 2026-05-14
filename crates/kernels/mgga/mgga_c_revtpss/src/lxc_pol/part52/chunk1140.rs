//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1140/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1140<F: Float>(t1459: F, t34366: F, t2113: F, t28265: F, t5795: F, t8731: F, t28268: F, t7334: F, t8118: F, t28280: F, t1916: F, t32779: F, t127495: F, t1461: F, t34011: F, t34014: F, t34341: F, t7324: F, t8127: F, t8616: F) -> (F,) {
    let t129095 = 6.0 * t1459 * t34366;
    let t129097 = 6.0 * t2113 * t28265;
    let t129099 = 6.0 * t5795 * t8731;
    let t129103 = 6.0 * t2113 * t28268;
    let t129107 = 3.0 * t8118 * t7334;
    let t129109 = 3.0 * t2113 * t28280;
    let t129111 = 6.0 * t1916 * t32779;
    let t129112 = 3.0 * t1461 * t34341 + 3.0 * t7324 * t8127 + t127495 + t129095 + t129097 + t129099 + t129103 + t129107 + t129109 + t129111 + t34011 + t34014 + t8616;
    (t129112,)
}
