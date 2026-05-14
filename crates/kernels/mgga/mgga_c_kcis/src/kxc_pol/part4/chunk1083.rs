//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1083/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1083<F: Float>(t14766: F, t14769: F, t14771: F, t14773: F, t14776: F, t14779: F, t14783: F, t14786: F, t14789: F, t14791: F, t14794: F, t14801: F, t14804: F, t14807: F, t14810: F, t14813: F, t14817: F, t14819: F, t14821: F, t14823: F, t14825: F, t14827: F, t14830: F, t14834: F, t14836: F, t14840: F, t14843: F, t14845: F, t14847: F, t14851: F, t14854: F) -> (F, F) {
    let t15739 = -0.4046875e-1 * t14766 + 0.20833333333333333333e-1 * t14769 - 0.125e0 * t14771 - 0.33333333333333333334e0 * t14773 - 0.45564814814814814814e0 * t14776 - 0.625e-1 * t14779 + 0.1875e0 * t14783 + 0.26979166666666666666e-1 * t14786 - 0.20833333333333333333e-1 * t14789 + 0.14388888888888888889e0 * t14791 - 0.41666666666666666666e-1 * t14794;
    let t15762 = 0.101171875e-1 * t14801 + 0.34173611111111111111e0 * t14804 + 0.21583333333333333334e0 * t14807 - 0.625e-1 * t14810 - 0.20234375e-1 * t14813 - 0.9375e-1 * t14817 + 0.5e0 * t14819 - 0.13489583333333333333e-1 * t14821 + 0.26979166666666666666e-1 * t14823 + 0.625e-1 * t14825 - 0.44965277777777777777e-2 * t14827 - 0.20234375e-1 * t14830 - 0.5625e0 * t14834 - 0.14388888888888888889e0 * t14836 + 0.27777777777777777777e-1 * t14840 - 0.44965277777777777777e-2 * t14843 + 0.101171875e-1 * t14845 - 0.125e0 * t14847 + 0.60703125e-1 * t14851 + 0.13489583333333333333e-1 * t14854;
    (t15739, t15762)
}
