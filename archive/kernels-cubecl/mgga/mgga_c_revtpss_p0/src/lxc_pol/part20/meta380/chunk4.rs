//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1382/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1382<F: Float>(t10794: F, t10811: F, t10807: F, t10709: F, t10760: F, t9794: F, t124: F, t138: F, t40649: F, t9645: F, t810: F, t10732: F) -> (F, F, F, F, F, F) {
    let t40748 = t10811 * t10794;
    let t40750 = t10811 * t10807;
    let t40753 = t10760 * t9794 * t10709;
    let t40757 = t138 * t124 * t40649 * t9645;
    let t40759 = F::cast_from(0.26776076960158126592e-7_f64) * t40757 * t810;
    let t40761 = t10760 * t9794 * t10732;
    (t40748, t40750, t40753, t40757, t40759, t40761)
}
