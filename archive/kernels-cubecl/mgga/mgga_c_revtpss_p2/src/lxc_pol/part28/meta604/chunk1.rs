//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2085/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2085<F: Float>(t94395: F, t97685: F, t14109: F, t25900: F, t94649: F, t1892: F, t786: F, t25877: F, t25881: F, t2028: F, t25931: F, t14224: F, t689: F) -> (F, F, F, F, F, F, F) {
    let t97687 = F::cast_from(0.28912093960683998208e-1_f64) * t94395 * t97685;
    let t97688 = t14109 * t25900;
    let t97690 = F::cast_from(0.28912093960683998208e-1_f64) * t94395 * t97688;
    let t97698 = F::cast_from(0.51405703062096148812e-1_f64) * t94649 * t97688;
    let t97699 = t786 * t1892;
    let t97700 = t97699 * t25877;
    let t97702 = F::cast_from(0.28912093960683998208e-1_f64) * t97700 * t25881;
    let t97703 = t2028 * t25931;
    let t97705 = t14224 * t689;
    (t97687, t97690, t97698, t97699, t97702, t97703, t97705)
}
