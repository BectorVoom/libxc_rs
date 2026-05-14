//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 651/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk651<F: Float>(t2063: F, t5136: F, t1049: F, t1849: F, t1835: F, t6714: F, t1842: F, t1856: F, t2494: F, t960: F, t2497: F, t965: F, t2502: F, t970: F, t6764: F, t6759: F, t706: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6906 = t5136 * t2063;
    let t6910 = t1049 * t1849;
    let t6913 = t1835 * t6714;
    let t6916 = t1842 * t6714;
    let t6919 = t1856 * t6714;
    let t6922 = t960 * t2494;
    let t6924 = t965 * t2497;
    let t6926 = t970 * t2502;
    let t6928 = t1835 * t6764;
    let t6931 = t706 * t6759;
    (t6906, t6910, t6913, t6916, t6919, t6922, t6924, t6926, t6928, t6931)
}
