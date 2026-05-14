//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1102/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1102<F: Float>(t1095: F, t1938: F, t5830: F, t1070: F, t5775: F, t5777: F, t1893: F, t1899: F, t7278: F, t17381: F, t20662: F, t20685: F, t20834: F, t20837: F, t20849: F, t20892: F, t20895: F, t20898: F, t20900: F, t20902: F, t20904: F, t5883: F, t5887: F, t5894: F, t721: F) -> (F, F, F) {
    let t20905 = t1938 * t1095;
    let t20908 = t5830 * t1095;
    let t20911 = t1070 * t5775;
    let t20913 = 0.96491876992155210402e2 * t20911 * t5777;
    let t20916 = 0.48245938496077605201e2 * t1899 * t7278 * t1893;
    let t20917 = -t20662 - 0.31168546390226634765e3 * t20834 * t5894 + 0.30762056574649219974e4 * t20837 * t17381 * t721 + t20685 - 0.19751673498613801407e-1 * t20849 - t20892 + t20895 - t20898 - t20900 - t20902 - t20904 + 18.0 * t20905 * t5883 - 0.57895126195293126243e3 * t20908 * t5887 + t20913 - t20916;
    (t20913, t20916, t20917)
}
