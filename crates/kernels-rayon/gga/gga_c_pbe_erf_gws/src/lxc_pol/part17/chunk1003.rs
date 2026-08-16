//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1003/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1003(t3116: f64, t6535: f64, t3139: f64, t875: f64, t8840: f64, t2168: f64, t2190: f64, t3131: f64, t1114: f64, t6671: f64, t6674: f64, t6414: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9007 = t3116 * t6535 / 24.0_f64;
    let t9009 = t3139 * t8840 * t875;
    let t9011 = t2168 * t9009 / 48.0_f64;
    let t9013 = t3139 * t3131 * t2190;
    let t9015 = t2168 * t9013 / 96.0_f64;
    let t9016 = t1114 * t6671;
    let t9018 = t9016 * t6674 / 24.0_f64;
    let t9019 = 7.0_f64 / 288.0_f64 * t6414;
    (t9007, t9009, t9011, t9013, t9015, t9018, t9019)
}
