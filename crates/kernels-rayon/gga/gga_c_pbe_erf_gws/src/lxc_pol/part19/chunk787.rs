//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 787/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk787(t542: f64, t671: f64, t670: f64, t1999: f64, t245: f64, t2003: f64, t1984: f64, t225: f64, t10: f64, t156: f64, t1996: f64, t2000: f64) -> (f64, f64, f64, f64, f64) {
    let t5917 = t542 * t671;
    let t5919 = 0.96187034332131941129e-1_f64 * t670 * t5917;
    let t5920 = t245 * t1999;
    let t5922 = 0.33545228223331014468e-1_f64 * t2003 * t5920;
    let t5926 = t225 * t1984;
    let t5927 = t10 * t5926;
    let t5929 = 0.32463124087094530131e0_f64 * t670 * t5927;
    let t5931 = t156 * t1999;
    let t5933 = 0.21642082724729686754e0_f64 * t670 * t5931;
    let t5938 = t1996 * t2000;
    (t5919, t5922, t5929, t5933, t5938)
}
