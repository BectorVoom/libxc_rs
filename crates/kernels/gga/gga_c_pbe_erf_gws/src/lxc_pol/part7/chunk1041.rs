//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1041/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1041<F: Float>(t20898: F, t821: F, t6184: F, t6217: F, t6538: F, t2313: F, t745: F, t2074: F, t2148: F, t337: F, t6560: F, t2146: F, t6535: F, t6702: F, t6258: F, t6711: F) -> (F, F, F, F, F, F, F) {
    let t20899 = t821 * t20898;
    let t20903 = t6217 * t6184;
    let t20904 = 7.0 / 24.0 * t20903;
    let t20905 = t6538 * t6184;
    let t20906 = 7.0 / 24.0 * t20905;
    let t20907 = t2313 * t745;
    let t20912 = t2148 * t2074;
    let t20914 = t6560 * t337 * t20912;
    let t20916 = 3.0 / 4.0 * t2146 * t20914;
    let t20919 = t6702 * t6535 / 6.0;
    let t20921 = t6711 * t6258 / 8.0;
    (t20899, t20904, t20906, t20907, t20916, t20919, t20921)
}
