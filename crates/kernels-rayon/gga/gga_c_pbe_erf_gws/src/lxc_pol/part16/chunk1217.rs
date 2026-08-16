//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1217/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1217(t51977: f64, t2242: f64, t4113: f64, t1208: f64, t6729: f64, t1206: f64, t2100: f64, t353: f64, t859: f64, t14182: f64, t19906: f64, t4083: f64, t4474: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52582 = 455.0_f64 / 648.0_f64 * t51977;
    let t52586 = t2242 * t4113;
    let t52589 = 455.0_f64 / 1296.0_f64 * t6729 * t1208;
    let t52600 = t859 * t353 * t1206 * t2100;
    let t52603 = t19906 * t14182;
    let t52607 = t4474 * t4083;
    (t52582, t52586, t52589, t52600, t52603, t52607)
}
