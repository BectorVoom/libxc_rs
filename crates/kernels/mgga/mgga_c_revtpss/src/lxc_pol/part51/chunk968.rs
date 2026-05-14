//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 968/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk968<F: Float>(t32117: F, t7898: F, t28187: F, t8568: F, t33913: F, t7239: F, t33597: F, t7235: F, t32110: F, t7732: F, t121441: F, t2014: F, t7900: F, t33667: F, t32121: F, t32103: F) -> (F, F, F, F, F, F, F, F, F) {
    let t125499 = t7898 * t32117;
    let t125500 = t8568 * t28187;
    let t125502 = t33913 * t7239;
    let t125505 = 3.0 * t7235 * t33597;
    let t125507 = 2.0 * t7732 * t32110;
    let t125510 = 3.0 * t2014 * t121441 * t7900;
    let t125512 = 2.0 * t7235 * t33667;
    let t125514 = 3.0 * t7898 * t32121;
    let t125515 = t7732 * t32103;
    (t125499, t125500, t125502, t125505, t125507, t125510, t125512, t125514, t125515)
}
