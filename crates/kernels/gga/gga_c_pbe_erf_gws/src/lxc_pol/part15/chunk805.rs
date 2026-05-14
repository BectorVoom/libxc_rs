//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 805/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk805<F: Float>(t1006: F, t1806: F, t1062: F, t1903: F, t7380: F, t4940: F, t4941: F, t4943: F, t4945: F, t4947: F, t7371: F, t7374: F, t7376: F, t7378: F, t7383: F, t7386: F, t7389: F, t7392: F, t7395: F, t7398: F, t7401: F) -> (F, F, F) {
    let t7540 = 4.0 / 15.0 * t1006 * t1806;
    let t7541 = t1062 * t1903;
    let t7549 = 0.2518888888888888889e-2 * t7380;
    let t7559 = t4940 + 0.16792592592592592593e-2 * t4941 - 0.41981481481481481482e-3 * t4943 + 0.12594444444444444445e-2 * t4945 - 0.62972222222222222223e-3 * t4947 + 0.83962962962962962964e-3 * t7374 - 0.83962962962962962965e-3 * t7378 + t7549 - 0.1385388888888888889e-1 * t7376 + 0.20990740740740740742e-2 * t7383 - 0.75566666666666666669e-2 * t7386 + 0.50377777777777777779e-2 * t7389 + 0.12594444444444444445e-2 * t7392 + 0.11335e-1 * t7395 - 0.15113333333333333334e-1 * t7398 - 0.37783333333333333334e-2 * t7401 + 0.37783333333333333334e-2 * t7371;
    (t7540, t7541, t7559)
}
