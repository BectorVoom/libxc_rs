//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 985/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk985<F: Float>(t2209: F, t3041: F, t2215: F, t3046: F, t836: F, t3052: F, t218: F, t3061: F, t675: F, t3065: F, t1167: F, t2185: F, t219: F, t3026: F, t824: F, t334: F, t7945: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t7970 = t3041 * t2209;
    let t7972 = t2215 * t3046;
    let t7973 = t7972 * t836;
    let t7975 = t3052 * t2209;
    let t7979 = t218 * t675 * t3061;
    let t7980 = 0.32862666666666666666e0 * t7979;
    let t7982 = t218 * t675 * t3065;
    let t7983 = 0.32862666666666666666e0 * t7982;
    let t7984 = t2185 * t1167;
    let t7986 = t218 * t219 * t7984;
    let t7988 = t824 * t3026;
    let t7990 = t218 * t219 * t7988;
    let t7992 = t334 * t7945;
    (t7970, t7973, t7975, t7979, t7980, t7982, t7983, t7984, t7986, t7988, t7990, t7992)
}
