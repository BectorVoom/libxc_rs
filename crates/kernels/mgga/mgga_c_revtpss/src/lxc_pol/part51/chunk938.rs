//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 938/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk938<F: Float>(t25669: F, t31999: F, t7150: F, t1045: F, t988: F, t31902: F, t8514: F, t93982: F, t32026: F, t3336: F, t11108: F, t8527: F, t41937: F, t8531: F, t32247: F, t32283: F) -> (F, F, F, F, F, F, F, F) {
    let t120708 = t7150 * t25669 * t31999;
    let t120709 = t1045 * t988;
    let t120714 = t31902 * t25669;
    let t120715 = t120714 * t31999;
    let t120724 = t8514 * t93982;
    let t120745 = t32026 * t3336;
    let t120749 = t8527 * t11108;
    let t120767 = t8531 * t41937;
    let t120952 = t32247 * t32283;
    (t120708, t120709, t120715, t120724, t120745, t120749, t120767, t120952)
}
