//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 999/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk999<F: Float>(t486: F, t931: F, t2411: F, t67: F, t1478: F, t405: F, t154: F, t385: F, t824: F, t178: F, t404: F, t4902: F, t17928: F, t2362: F, t326: F, t17932: F, t401: F) -> (F, F, F, F, F, F, F, F) {
    let t18989 = t486 * t931;
    let t18994 = t67 * t2411;
    let t19023 = t1478 * t405;
    let t19026 = t385 * t154 * t19023 * t824;
    let t19055 = 0.14820648238345094262e-3 * t404 * t178 * t4902 * t405;
    let t19078 = t17928 * t2362;
    let t19079 = t19078 * t326;
    let t19080 = t401 * t17932;
    (t18989, t18994, t19023, t19026, t19055, t19078, t19079, t19080)
}
