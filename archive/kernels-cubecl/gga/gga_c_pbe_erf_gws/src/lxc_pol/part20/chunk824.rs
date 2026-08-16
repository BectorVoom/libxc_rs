//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 824/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk824<F: Float>(t1697: F, t5212: F, t1802: F, t589: F, t1631: F, t2612: F, t2740: F, t586: F, t2636: F, t5018: F, t1820: F, t1062: F, t1903: F) -> (F, F, F, F, F, F) {
    let t7505 = t5212 * t1697;
    let t7514 = t589 * t1802;
    let t7526 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t2612 * t1631;
    let t7527 = t2740 * t586;
    let t7530 = t5018 * t2636;
    let t7532 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1820 * t7530;
    let t7541 = t1062 * t1903;
    (t7505, t7514, t7526, t7527, t7532, t7541)
}
