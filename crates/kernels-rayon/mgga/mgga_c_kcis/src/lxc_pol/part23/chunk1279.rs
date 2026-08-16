//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1279/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1279(t16685: f64, t27369: f64, t27396: f64, t27416: f64, t27453: f64, t28480: f64, t3984: f64, t52402: f64, t5709: f64, t7901: f64, t7908: f64, t7909: f64, t8144: f64, t94651: f64, t98081: f64, t98087: f64, t98883: f64, t98888: f64, t98903: f64) -> f64 {
    let t98906 = 0.69505208333333333333e-3_f64 * t8144 * t27416 + 0.30891203703703703704e-3_f64 * t94651 - 0.49745833333333333332e-2_f64 * t98883 - 0.37069444444444444444e-2_f64 * t28480 * t7901 + t98888 + 0.23168402777777777778e-3_f64 * t7908 * t3984 * t7909 * t52402 + 0.23168402777777777778e-3_f64 * t7908 * t98081 - 0.92754700520833333335e-4_f64 * t27369 * t98087 + 0.46336805555555555556e-3_f64 * t7908 * t5709 * t27453 * t16685 + t98903 - 0.13901041666666666667e-2_f64 * t8144 * t27396;
    t98906
}
