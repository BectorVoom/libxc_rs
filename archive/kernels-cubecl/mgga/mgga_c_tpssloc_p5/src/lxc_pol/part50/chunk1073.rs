//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1073/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1073<F: Float>(t1378: F, t31216: F, t1375: F, t1386: F, t2016: F, t22656: F, t22670: F, t31140: F, t31147: F, t31151: F, t31182: F, t31189: F, t3758: F, t3882: F, t568: F, t6958: F, t6963: F, t8476: F, t8486: F) -> (F, F) {
    let t31217 = t1378 * t31216;
    let t31219 = -t1375 * t31217 - t1386 * t31189 - F::cast_from(2.0_f64) * t2016 * t22656 - F::cast_from(2.0_f64) * t2016 * t22670 + t31151 * t568 + t31182 * t568 + F::cast_from(2.0_f64) * t3758 * t8476 - t3758 * t8486 + F::cast_from(2.0_f64) * t3882 * t8476 - t3882 * t8486 + F::cast_from(4.0_f64) * t6958 * t6963 - t31140 - t31147;
    (t31217, t31219)
}
