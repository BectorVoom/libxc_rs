//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1083/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1083(t18113: f64, t19103: f64, t19170: f64, t19469: f64, t312: f64, t16386: f64, t18424: f64, t18428: f64, t18432: f64, t18435: f64, t18439: f64, t18441: f64, t18445: f64, t18448: f64, t18452: f64, t18456: f64, t18460: f64, t18462: f64, t18467: f64, t2182: f64, t2423: f64, t2424: f64, t2429: f64, t321: f64) -> (f64, f64) {
    let t19472 = (t18113 + t19103 + t19170 + t19469) * t312;
    let t19476 = 12.0_f64 * t16386 * t2423 * t321 + 36.0_f64 * t2182 * t2424 * t2429 + t18424 - t18428 + t18432 - t18435 + t18439 - t18441 - t18445 - t18448 - t18452 + t18456 - t18460 - t18462 + t18467 - t19472;
    (t19472, t19476)
}
