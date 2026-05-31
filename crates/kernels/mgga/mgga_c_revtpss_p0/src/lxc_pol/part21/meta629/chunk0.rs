//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2394/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2394<F: Float>(t222: F, t40735: F, t10777: F, t10779: F, t2749: F, t40578: F, t10794: F, t10811: F, t10807: F, t10709: F, t10760: F, t9794: F) -> (F, F, F, F, F) {
    let t40737 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t40735 * t222;
    let t40744 = t10777 * t10779 * t40578 * t2749;
    let t40748 = t10811 * t10794;
    let t40750 = t10811 * t10807;
    let t40753 = t10760 * t9794 * t10709;
    (t40737, t40744, t40748, t40750, t40753)
}
