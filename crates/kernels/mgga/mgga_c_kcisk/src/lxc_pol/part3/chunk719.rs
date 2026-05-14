//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 719/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk719<F: Float>(t12128: F, t12142: F, t10508: F, t10513: F, t10515: F, t10517: F, t10525: F, t10527: F, t10530: F, t10532: F, t10537: F, t11209: F, t11211: F, t11216: F, t11967: F, t11983: F, t11986: F, t11991: F, t1994: F, t5348: F, t5440: F, t5445: F, t795: F) -> (F, F) {
    let t12143 = t12128 + t12142;
    let t12156 = t11983 - 0.43134342e-1 * t11986 * t11967 + 0.579e0 * t5348 * t5440 + 0.579e0 * t1994 * t11991 + 0.34822083333333333333e-2 * t10508 - 0.52233124999999999998e-2 * t10513 + 0.23214722222222222222e-2 * t10515 + t12143 * t795 - 0.11607361111111111111e-2 * t10517 + 0.11607361111111111111e-2 * t10525 + 0.46429444444444444443e-2 * t10527 - 0.11607361111111111111e-2 * t10530 - 0.34822083333333333333e-2 * t10532 + 0.223494e0 * t5445 * t11991 - 0.34822083333333333333e-2 * t10537 + 0.11607361111111111111e-2 * t11209 - 0.77382407407407407405e-3 * t11211 + 0.51588271604938271604e-3 * t11216;
    (t12143, t12156)
}
