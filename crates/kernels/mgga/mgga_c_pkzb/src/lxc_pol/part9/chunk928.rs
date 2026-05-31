//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 928/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk928<F: Float>(t568: F, t7074: F, t1692: F, t2632: F, t596: F, t6853: F, t1029: F, t1031: F, t160: F, t162: F, t1742: F, t1747: F, t1750: F, t2625: F, t2631: F, t2633: F, t2636: F, t594: F, t597: F, t7055: F, t7065: F, t7071: F) -> (F, F, F, F) {
    let t7075 = t7074 * t568;
    let t7078 = t2632 * t1692;
    let t7081 = t596 * t6853;
    let t7084 = -F::cast_from(12.0_f64) * t1029 * t1747 + F::cast_from(3.0_f64) * t1029 * t1750 + F::cast_from(3.0_f64) * t1031 * t1742 + F::cast_from(3.0_f64) * t160 * t7081 - t162 * t7055 + F::cast_from(6.0_f64) * t2625 * t597 + F::cast_from(60.0_f64) * t2631 * t7071 - F::cast_from(24.0_f64) * t2631 * t7075 - F::cast_from(12.0_f64) * t2631 * t7078 - F::cast_from(24.0_f64) * t2633 * t7065 + F::cast_from(6.0_f64) * t2636 * t594;
    (t7075, t7078, t7081, t7084)
}
