//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1060/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1060<F: Float>(t13083: F, t3360: F, t4284: F, t1434: F, t3700: F, t1165: F, t15407: F, t3456: F, t540: F, t12727: F, t1470: F, t1137: F, t4594: F) -> (F, F, F, F, F) {
    let t18686 = t3360 * t13083 * t4284;
    let t18690 = t3700 * t1434;
    let t18702 = t3456 * t1165 * t540 * t15407;
    let t18704 = t12727 * t1470;
    let t18719 = t1137 * t4594;
    (t18686, t18690, t18702, t18704, t18719)
}
