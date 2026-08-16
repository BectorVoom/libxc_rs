//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 993/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk993(t1086: f64, t7739: f64, t11990: f64, t2597: f64, t7503: f64, t11320: f64, t325: f64, t11938: f64, t11183: f64, t11186: f64, t11190: f64, t11193: f64, t11196: f64, t11200: f64, t11205: f64, t11212: f64, t11218: f64, t11220: f64, t11225: f64, t11229: f64, t11231: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11991 = t1086 * t7739;
    let t11992 = t11990 * t11991;
    let t11994 = t2597 * t7503;
    let t11995 = t11990 * t11994;
    let t11997 = t325 * t11320;
    let t11998 = t11997 * t11938;
    let t12312 = 0.10862280351692200478e-4_f64 * t11183 + 0.10862280351692200478e-4_f64 * t11186 - 0.2429468532550759923e-3_f64 * t11190 - 0.2429468532550759923e-3_f64 * t11193 - 0.809822844183586641e-4_f64 * t11196 + 0.17379648562707520765e-4_f64 * t11200 + 0.50613927761474165061e-5_f64 * t11205 - 0.36207601172307334926e-6_f64 * t11212 + 0.47522476538653377091e-5_f64 * t11218 - 0.17379648562707520765e-3_f64 * t11220 - 0.17379648562707520765e-3_f64 * t11225 + 0.50613927761474165061e-5_f64 * t11229 + 0.6951859425083008306e-3_f64 * t11231;
    (t11991, t11992, t11994, t11995, t11997, t11998, t12312)
}
