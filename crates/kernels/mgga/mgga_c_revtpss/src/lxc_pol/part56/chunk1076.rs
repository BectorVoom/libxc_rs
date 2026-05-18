//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1076/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1076<F: Float>(t1276: F, t2142: F, t2148: F, t3140: F, t26904: F, t33468: F, t33517: F, t33494: F, t97040: F, t26894: F, t33508: F, t31993: F, t33524: F, t3635: F) -> (F, F, F, F, F) {
    let t124780 = t2148 * t2142 * t3140 * t1276;
    let t124801 = t33468 * t26904;
    let t124802 = t124801 * t33517;
    let t124814 = t97040 * t33494;
    let t124819 = t26894 * t33508;
    let t124825 = t33524 * t31993 * t3635;
    (t124780, t124802, t124814, t124819, t124825)
}
