//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 904/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk904<F: Float>(t1820: F, t7888: F, t1027: F, t1733: F, t1809: F, t1620: F, t1407: F, t2579: F, t1821: F, t1663: F, t995: F, t1403: F) -> (F, F, F, F) {
    let t7890 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t1820 * t7888;
    let t7891 = t1027 * t1733;
    let t7892 = t1809 * t7891;
    let t7894 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1620 * t7892;
    let t7895 = t2579 * t1407;
    let t7896 = t1821 * t7895;
    let t7898 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1820 * t7896;
    let t7899 = t995 * t1663;
    let t7900 = t7899 * t1403;
    (t7890, t7894, t7898, t7900)
}
