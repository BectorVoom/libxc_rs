//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1061/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1061<F: Float>(t26399: F, t7742: F, t28658: F, t28063: F, t7359: F, t34018: F, t7235: F, t34302: F, t95088: F, t2014: F, t28176: F, t32629: F, t198: F, t205: F, t8656: F, t2411: F, t34079: F) -> (F, F, F, F, F, F, F, F) {
    let t127545 = 2.0 * t26399 * t7742;
    let t127547 = 2.0 * t28658 * t7742;
    let t127549 = 2.0 * t7359 * t28063;
    let t127550 = t7235 * t34018;
    let t127556 = 3.0 * t95088 * t34302;
    let t127559 = 3.0 * t2014 * t32629 * t28176;
    let t127566 = t198 * t205 * t8656;
    let t127582 = t34079 * t2411;
    (t127545, t127547, t127549, t127550, t127556, t127559, t127566, t127582)
}
