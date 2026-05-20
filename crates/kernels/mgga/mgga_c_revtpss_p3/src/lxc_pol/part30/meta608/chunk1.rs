//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2073/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2073<F: Float>(t25875: F, t97676: F, t97680: F, t1444: F, t5740: F, t675: F, t685: F, t94395: F, t14109: F, t25900: F, t94649: F, t1892: F, t786: F) -> (F, F, F, F, F, F) {
    let t97682 = F::cast_from(0.51405703062096148812e-1_f64) * t25875 * t97676 * t97680;
    let t97685 = t5740 * t685 * t675 * t1444;
    let t97687 = F::cast_from(0.28912093960683998208e-1_f64) * t94395 * t97685;
    let t97688 = t14109 * t25900;
    let t97690 = F::cast_from(0.28912093960683998208e-1_f64) * t94395 * t97688;
    let t97698 = F::cast_from(0.51405703062096148812e-1_f64) * t94649 * t97688;
    let t97699 = t786 * t1892;
    (t97682, t97685, t97687, t97690, t97698, t97699)
}
