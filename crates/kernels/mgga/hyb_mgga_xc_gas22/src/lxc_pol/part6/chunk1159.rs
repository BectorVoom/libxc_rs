//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1159/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1159<F: Float>(t1788: F, t555: F, t6160: F, t10: F, t19706: F, t1897: F, t28: F, t1806: F, t1815: F, t1804: F, t1809: F, t1797: F) -> (F, F, F, F, F, F) {
    let t20073 = t555 * t6160 * t1788;
    let t20075 = t19706 * t10;
    let t20078 = F::cast_from(1.0_f64) / t28 / t1897;
    let t20127 = t1815 * t1806;
    let t20129 = t1804 * t20127 * t1809;
    let t20132 = t555 * t6160 * t1797;
    (t20073, t20075, t20078, t20127, t20129, t20132)
}
