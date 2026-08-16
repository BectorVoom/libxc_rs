//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1173/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1173(t34488: f64, t34500: f64, t34506: f64, t34508: f64, t34510: f64, t34512: f64, t30601: f64, t30605: f64, t30607: f64, t30611: f64, t30613: f64, t32517: f64, t34482: f64, t34492: f64, t34497: f64, t34502: f64, t34504: f64, t34516: f64) -> f64 {
    let t37121 = 0.916875e-1_f64 * t34488;
    let t37126 = 0.68598428988911579156e-2_f64 * t34500;
    let t37129 = 0.34299214494455789578e-2_f64 * t34506;
    let t37130 = 0.31448092289604152068e-2_f64 * t34508;
    let t37131 = 0.13208198761633743869e-1_f64 * t34510;
    let t37132 = 0.32012600194825403606e-1_f64 * t34512;
    let t37134 = -0.51448821741683684367e-2_f64 * t34482 + t32517 - t30601 / 32.0_f64 - t30605 / 96.0_f64 + 0.28015625e-1_f64 * t30607 + t37121 - 0.31448092289604152069e-3_f64 * t34492 - 0.51448821741683684368e-2_f64 * t30611 + 0.12579236915841660828e-2_f64 * t34497 - 0.51448821741683684368e-2_f64 * t30613 + t37126 - 0.34299214494455789578e-2_f64 * t34502 - 0.17149607247227894789e-2_f64 * t34504 + t37129 - t37130 + t37131 - t37132 + 0.41930789719472202759e-2_f64 * t34516;
    t37134
}
