//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 850/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk850<F: Float>(t1830: F, t6808: F, t2546: F, t350: F, t506: F, t6402: F, t36: F, t6406: F, t2550: F, t1476: F, t6765: F, t6760: F, t497: F, t5974: F, t2900: F, t2901: F, t4878: F, t4911: F, t4916: F, t5405: F, t6800: F, t6803: F, t6806: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6809 = t1830 * t6808;
    let t6811 = t350 * t2546;
    let t6813 = t506 * t6402;
    let t6814 = t36 * t6813;
    let t6816 = t506 * t6406;
    let t6817 = t1830 * t6816;
    let t6819 = t350 * t2550;
    let t6821 = t1476 * t6765;
    let t6822 = t36 * t6821;
    let t6824 = t506 * t6760;
    let t6825 = t36 * t6824;
    let t6827 = t497 * t5974;
    let t6828 = t506 * t6827;
    let t6829 = t36 * t6828;
    let t6831 = t2900 + 0.0008396296296296296 * t2901 + 0.0016792592592592592 * t4911 - 0.0008396296296296296 * t4878 + t5405 + 0.002518888888888889 * t4916 - 0.0004198148148148148 * t6800 + 0.002099074074074074 * t6803 - 0.007556666666666666 * t6806 - 0.005037777777777778 * t6809 + 0.0012594444444444445 * t6811 + 0.011335 * t6814 + 0.015113333333333333 * t6817 - 0.0006297222222222223 * t6819 + 0.0012594444444444445 * t6822 - 0.003778333333333333 * t6825 + 0.0018891666666666666 * t6829;
    (t6809, t6811, t6813, t6814, t6816, t6817, t6819, t6821, t6822, t6824, t6825, t6827, t6828, t6829, t6831)
}
