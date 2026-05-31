//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1351/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1351<F: Float>(t24702: F, t8614: F, t8621: F, t8906: F, t24712: F, t8625: F, t2228: F, t2234: F, t4114: F, t2189: F, t4143: F, t6562: F) -> (F, F, F, F, F) {
    let t29438 = F::cast_from(0.19298375398431042081e3_f64) * t24702 * t8614;
    let t29440 = F::cast_from(0.32163958997385070134e2_f64) * t8906 * t8621;
    let t29442 = F::cast_from(0.1034520258385468006e4_f64) * t24712 * t8625;
    let t29445 = F::cast_from(6.0_f64) * t2234 * t4114 * t2228;
    let t29448 = F::cast_from(0.57895126195293126241e3_f64) * t6562 * t4143 * t2189;
    (t29438, t29440, t29442, t29445, t29448)
}
