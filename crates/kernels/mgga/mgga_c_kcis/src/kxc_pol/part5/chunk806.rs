//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 806/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk806<F: Float>(t1335: F, t6985: F, t1316: F, t3901: F, t6953: F, t3899: F, t3905: F, t5469: F, t6939: F, t6942: F, t6946: F, t482: F, t1919: F) -> (F, F, F, F, F, F, F) {
    let t6986 = t6985 * t1335;
    let t6988 = 1.0 * t1316 * t6986;
    let t6989 = t6953 * t3901;
    let t6991 = 0.16081824322151104822e2 * t3899 * t6989;
    let t6996 = t3905 + 0.61805555555555555556e-2 * t5469 - 0.61805555555555555555e-2 * t6939 + 0.18541666666666666667e-1 * t6942 - 0.92708333333333333333e-2 * t6946;
    let t6997 = t6996 * t482;
    let t7002 = t1919 * t1919;
    (t6986, t6988, t6989, t6991, t6996, t6997, t7002)
}
