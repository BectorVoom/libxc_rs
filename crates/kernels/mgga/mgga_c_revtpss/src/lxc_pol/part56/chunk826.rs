//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 826/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk826<F: Float>(t2014: F, t32117: F, t531: F, t8598: F, t1353: F, t4147: F, t1448: F, t9593: F, t8599: F, t10301: F, t8435: F, t644: F, t8441: F, t8621: F, t36: F, t606: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32118 = t2014 * t32117;
    let t32119 = t531 * t8598;
    let t32120 = t4147 * t1353;
    let t32121 = t32119 * t32120;
    let t32123 = 3.0 * t2014 * t32121;
    let t32128 = t9593 * t1448;
    let t32129 = t8599 * t32128;
    let t32131 = 2.0 * t2014 * t32129;
    let t32132 = t10301 * t8435;
    let t32137 = t8441 * t644;
    let t32138 = t8621 * t32137;
    let t32143 = t8441 * t36;
    let t32145 = t8621 * t32143 * t606;
    (t32118, t32119, t32121, t32123, t32129, t32131, t32132, t32138, t32143, t32145)
}
