//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 601/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk601<F: Float>(t4726: F, t6759: F, t26: F, t1659: F, t6764: F, t6714: F, t5744: F, t4638: F, t4711: F, t4723: F, t4724: F, t6756: F, t6761: F, t6766: F, t6769: F, t6778: F, t6780: F, t6818: F, t6820: F, t6823: F) -> (F, F, F, F, F, F, F) {
    let t6825 = t4726 * t6759;
    let t6826 = t26 * t6825;
    let t6828 = t1659 * t6764;
    let t6829 = t26 * t6828;
    let t6831 = t1659 * t6714;
    let t6832 = t5744 * t6831;
    let t6834 = -0.9494625e0 * t6778 + 0.1898925e1 * t6780 + t4711 + 0.99655555555555555557e-1 * t4638 + 0.99655555555555555557e-1 * t6756 - 0.19931111111111111111e0 * t6761 + 0.59793333333333333334e0 * t6766 + 0.59793333333333333334e0 * t6769 + 0.15358125e0 * t6818 + 0.3071625e0 * t6820 + t4723 + 0.54771111111111111111e-1 * t4724 + 0.54771111111111111111e-1 * t6823 - 0.27385555555555555556e-1 * t6826 + 0.16431333333333333333e0 * t6829 + 0.16431333333333333333e0 * t6832;
    (t6825, t6826, t6828, t6829, t6831, t6832, t6834)
}
