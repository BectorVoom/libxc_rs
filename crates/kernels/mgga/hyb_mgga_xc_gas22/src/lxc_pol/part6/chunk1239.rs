//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1239/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1239<F: Float>(t10617: F, t2183: F, t20691: F, t20697: F, t28794: F, t28797: F, t28800: F, t28804: F, t28808: F, t28837: F, t28840: F, t28844: F, t28847: F, t20694: F, t20703: F, t20706: F, t20853: F, t20867: F, t28850: F, t28853: F, t28856: F, t28859: F, t28862: F, t28866: F, t28872: F) -> (F, F, F) {
    let t28973 = 2.0 * t2183 * t10617;
    let t28985 = -0.1460562962962962963e1 * t20691 + 0.27385555555555555556e0 * t20697 + 0.27385555555555555555e0 * t28794 - 0.32862666666666666666e0 * t28797 - 0.32862666666666666666e0 * t28800 + 0.24647e0 * t28804 + 0.49294e0 * t28808 + 0.24647e0 * t28837 - 0.65725333333333333333e0 * t28840 + 0.49294e0 * t28844 - 0.1898925e1 * t28847;
    let t28996 = 0.3071625e0 * t28850 - 0.59793333333333333334e0 * t28853 + 0.8969e0 * t28856 + 0.39862222222222222223e0 * t28859 + 0.27385555555555555555e0 * t28862 + 0.49294e0 * t28866 + t20867 + 0.27385555555555555556e0 * t20694 + t20853 - 0.18602370370370370371e1 * t20703 + 0.39862222222222222223e0 * t20706 + 0.142419375e1 * t28872;
    (t28973, t28985, t28996)
}
