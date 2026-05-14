//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1022/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1022<F: Float>(t22944: F, t19191: F, t2380: F, t3195: F, t178: F, t19080: F, t19079: F, t19091: F, t2401: F, t3185: F, t3188: F, t3174: F, t3176: F, t487: F, t1228: F, t300: F) -> (F, F, F, F, F, F, F, F) {
    let t22945 = 0.14291339372689912324e-3 * t22944;
    let t22950 = t2380 * t19191 * t3195;
    let t22951 = 0.28582678745379824648e-3 * t22950;
    let t22971 = t19080 * t178;
    let t22972 = t19079 * t22971;
    let t22979 = t19091 * t22971;
    let t22988 = t3185 * t2401 * t3188;
    let t22989 = 0.28582678745379824648e-3 * t22988;
    let t23007 = t3174 * t487 * t3176;
    let t23008 = t23007 / 72.0;
    let t23054 = t300 * t1228;
    (t22945, t22951, t22971, t22972, t22979, t22989, t23008, t23054)
}
