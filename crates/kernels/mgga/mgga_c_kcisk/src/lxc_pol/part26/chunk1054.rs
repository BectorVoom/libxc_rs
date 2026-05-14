//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1054/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1054<F: Float>(t4350: F, t8403: F, t1596: F, t6587: F, t6591: F, t14639: F, t1557: F, t19926: F, t19948: F, t21969: F, t26882: F, t26885: F, t26892: F, t26896: F, t26899: F, t26903: F, t26907: F, t26912: F, t26914: F, t26917: F, t27959: F, t4347: F, t6426: F, t6592: F, t8289: F) -> (F, F, F) {
    let t27965 = t8403 * t4350;
    let t27966 = t27965 * t1596;
    let t27987 = t6591 * t6587;
    let t27992 = -0.386e0 * t1557 * t27959 + 0.193e0 * t1557 * t27966 + 0.74498e-1 * t4347 * t27966 + 0.74498e-1 * t14639 * t8289 - 0.77382407407407407407e-3 * t26882 - 0.23214722222222222222e-2 * t26885 - 0.51588271604938271603e-3 * t19926 + 0.38691203703703703703e-3 * t26892 + 0.51588271604938271604e-3 * t26896 + 0.23214722222222222222e-2 * t26899 + 0.386e0 * t6426 * t6592 - 0.25794135802469135802e-3 * t26903 - 0.92858888888888888885e-2 * t26907 - 0.51588271604938271603e-3 * t19948 + 0.148996e0 * t21969 * t6592 + 0.25794135802469135802e-2 * t26912 + 0.148996e0 * t4347 * t27987 - 0.25794135802469135802e-3 * t26914 + 0.34822083333333333332e-2 * t26917;
    (t27965, t27987, t27992)
}
