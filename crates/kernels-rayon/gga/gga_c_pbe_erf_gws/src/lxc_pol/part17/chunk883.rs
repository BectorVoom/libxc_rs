//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 883/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk883(t1885: f64, t7641: f64, t1820: f64, t1866: f64, t2630: f64, t587: f64, t1010: f64, t5304: f64, t1022: f64, t1697: f64, t1413: f64, t1809: f64) -> (f64, f64, f64, f64) {
    let t7642 = t1885 * t7641;
    let t7644 = 8.0_f64 / 15.0_f64 * t1820 * t7642;
    let t7645 = t2630 * t1866;
    let t7646 = t1885 * t7645;
    let t7648 = 4.0_f64 / 15.0_f64 * t587 * t7646;
    let t7650 = 8.0_f64 / 45.0_f64 * t5304 * t1010;
    let t7651 = t1022 * t1697;
    let t7652 = t7651 * t1413;
    let t7653 = t1809 * t7652;
    (t7644, t7648, t7650, t7653)
}
