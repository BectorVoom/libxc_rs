//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 720/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk720(t3882: f64, t8392: f64, t3888: f64, t1882: f64, t3979: f64, t13746: f64, t13753: f64, t13780: f64, t13794: f64, t13809: f64, t13811: f64, t3861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13961 = 2.0_f64 / 27.0_f64 * t8392 * t3882;
    let t13963 = 4.0_f64 / 27.0_f64 * t8392 * t3888;
    let t13965 = 2.0_f64 / 9.0_f64 * t1882 * t3979;
    let t13983 = 4.0_f64 / 3.0_f64 * t13746;
    let t13984 = 2.0_f64 / 3.0_f64 * t13753;
    let t13993 = 2.0_f64 / 9.0_f64 * t13780;
    let t13998 = 4.0_f64 / 27.0_f64 * t13794;
    let t14004 = 2.0_f64 / 9.0_f64 * t13809;
    let t14005 = 4.0_f64 / 9.0_f64 * t13811;
    let t14018 = 2.0_f64 / 9.0_f64 * t1882 * t3861;
    (t13961, t13963, t13965, t13983, t13984, t13993, t13998, t14004, t14005, t14018)
}
