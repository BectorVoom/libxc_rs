//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 835/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk835<F: Float>(t27: F, t3027: F, t545: F, t1377: F, t1403: F, t97: F, t1410: F, t1767: F, t184: F, t186: F, t30: F, t32: F, t1369: F, t3309: F, t1372: F, t1375: F, t740: F, t934: F) -> (F, F, F, F, F, F, F) {
    let t10757 = t3027 * t27 * t545;
    let t10760 = t1403 * t97 * t1377;
    let t10764 = 0.06709045644666203 * t1410 * t97 * t1377;
    let t10769 = 2.8503734567901235e-05 * t184 * t1767 * t30 * t32 * t186;
    let t10770 = t1369 * t3309;
    let t10773 = 0.38474813732852775 * t1372 * t3309;
    let t10777 = 0.019878653761973935 * t1375 * t934 * t740 * t186;
    (t10757, t10760, t10764, t10769, t10770, t10773, t10777)
}
