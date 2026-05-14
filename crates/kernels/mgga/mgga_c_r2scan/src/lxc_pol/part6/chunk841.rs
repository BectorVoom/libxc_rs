//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 841/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk841<F: Float>(t1783: F, t424: F, t2050: F, t2055: F, t761: F, t2054: F, t58: F, t423: F, t2056: F, t597: F, t1375: F, t2060: F, t2062: F, t759: F, t2049: F, t607: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6021 = t424 * t1783;
    let t6026 = 0.1714584e0 * t2055 * t2050 * t761;
    let t6027 = t2054 * t58;
    let t6028 = t6027 * t423;
    let t6029 = t597 * t2056;
    let t6030 = t6028 * t6029;
    let t6032 = t2060 * t1375;
    let t6033 = t6032 * t2062;
    let t6036 = t759 * t1783 * t761;
    let t6038 = t607 * t2049;
    (t6021, t6026, t6027, t6028, t6029, t6030, t6032, t6033, t6036, t6038)
}
