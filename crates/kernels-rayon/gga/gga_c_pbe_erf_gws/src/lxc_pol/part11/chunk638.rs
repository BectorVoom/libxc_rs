//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 638/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk638(t1985: f64, t226: f64, t163: f64, t4577: f64, t148: f64, t147: f64, t413: f64) -> (f64, f64, f64, f64) {
    let t5952 = 4.0_f64 * t226 * t1985;
    let t5975 = t4577 * t163;
    let t5977 = 0.31505407223141117834e-1_f64 * t148 * t5975;
    let t5984 = t413 * t147;
    (t5952, t5975, t5977, t5984)
}
