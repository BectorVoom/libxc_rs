//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1203/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1203<F: Float>(t2783: F, t5766: F, t1850: F, t7444: F, t1095: F, t1938: F, t5830: F, t1070: F, t5775: F, t5777: F, t1893: F, t1899: F, t7278: F) -> (F, F, F, F, F, F) {
    let t20902 = F::new(3.0) * t5766 * t2783;
    let t20904 = F::new(3.0) * t1850 * t7444;
    let t20905 = t1938 * t1095;
    let t20908 = t5830 * t1095;
    let t20911 = t1070 * t5775;
    let t20913 = F::cast_from(0.96491876992155210402e2_f64) * t20911 * t5777;
    let t20916 = F::cast_from(0.48245938496077605201e2_f64) * t1899 * t7278 * t1893;
    (t20902, t20904, t20905, t20908, t20913, t20916)
}
