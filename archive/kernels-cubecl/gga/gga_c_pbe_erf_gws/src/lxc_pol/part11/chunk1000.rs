//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1000/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1000<F: Float>(t20962: F, t3820: F, t11609: F, t4395: F, t6472: F, t11924: F, t20550: F, t3875: F, t6505: F, t3857: F, t6455: F, t20189: F, t3116: F, t3792: F) -> (F, F, F, F, F, F, F) {
    let t38735 = t20962 * t3820;
    let t38761 = t4395 * t11609;
    let t38850 = t6472 * t11609;
    let t38870 = t20550 * t11924;
    let t38979 = t6505 * t3875;
    let t38981 = t6455 * t3857;
    let t38997 = t3116 * t20189 * t3792;
    (t38735, t38761, t38850, t38870, t38979, t38981, t38997)
}
