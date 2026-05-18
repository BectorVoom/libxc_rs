//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1217/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1217<F: Float>(t6238: F, t899: F, t922: F, t2250: F, t3969: F, t933: F, t14022: F, t828: F, t2123: F, t2209: F, t4021: F, t916: F) -> (F, F, F, F, F, F) {
    let t51301 = t899 * t6238 * t922;
    let t51306 = t2250 * t3969 * t933;
    let t51328 = t14022 * t828;
    let t51329 = t51328 * t2123;
    let t51334 = t4021 * t2209;
    let t51350 = t3969 * t916;
    (t51301, t51306, t51328, t51329, t51334, t51350)
}
