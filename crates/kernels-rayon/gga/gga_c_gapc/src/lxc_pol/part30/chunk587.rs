//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 587/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk587(t1112: f64, t687: f64, t2970: f64, t2976: f64, t2984: f64, t2988: f64, t2991: f64, t3002: f64, t3009: f64, t3014: f64, t3019: f64, t3023: f64, t3025: f64) -> (f64, f64) {
    let t3483 = t1112 * t687;
    let t3497 = 0.10821235962619981449e-3_f64 * t2970 + 0.12163329537032409896e-2_f64 * t2976 - 0.20241536458333333335e-4_f64 * t2984 + 0.17376185052903442709e-3_f64 * t2988 + 0.17376185052903442709e-3_f64 * t2991 + 0.16882592796244404291e-6_f64 * t3002 + 0.33765185592488808582e-6_f64 * t3009 - 0.50680539737635041235e-4_f64 * t3014 - 0.14492726735651760868e-5_f64 * t3019 + 0.28985453471303521736e-5_f64 * t3023 - 0.16908181191593721013e-4_f64 * t3025;
    (t3483, t3497)
}
