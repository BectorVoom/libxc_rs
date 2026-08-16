//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1802/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1802<F: Float>(t2055: F, t5517: F, t72: F, t8094: F, t686: F, t25878: F, t25895: F, t1882: F, t543: F, t7506: F, t7301: F, t27884: F, t7515: F) -> (F, F, F, F, F, F, F, F) {
    let t28760 = t5517 * t2055;
    let t28779 = t8094 * t72;
    let t28780 = t28779 * t686;
    let t28781 = t25878 * t28780;
    let t28783 = t25895 * t28780;
    let t28791 = t7506 * t1882 * t543;
    let t28792 = t7301 * t28791;
    let t28796 = t27884 * t7515;
    (t28760, t28779, t28780, t28781, t28783, t28791, t28792, t28796)
}
