//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 776/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk776<F: Float>(t616: F, t7945: F, t202: F, t2814: F, t184: F, t1672: F, t996: F, t561: F, t2799: F, t7776: F, t2575: F, t4934: F, t1620: F, t2826: F, t583: F, t1076: F, t1365: F, t153: F) -> (F, F, F, F, F, F, F) {
    let t7946 = t616 * t7945;
    let t7950 = t202 * t2814;
    let t7951 = t7950 * t184;
    let t7956 = t1672 * t996;
    let t7957 = t561 * t7956;
    let t7959 = t7776 * t2799;
    let t7960 = t561 * t7959;
    let t7966 = t4934 * t2575;
    let t7968 = 32.0 / 135.0 * t1620 * t7966;
    let t7970 = 8.0 / 45.0 * t2826 * t583;
    let t7981 = t153 * t1365 * t1076;
    (t7946, t7951, t7957, t7960, t7968, t7970, t7981)
}
