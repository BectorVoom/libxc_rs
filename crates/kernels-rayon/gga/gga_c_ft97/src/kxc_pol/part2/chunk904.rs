//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 904/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk904(t13794: f64, t13764: f64, t13768: f64, t13772: f64, t13775: f64, t13778: f64, t13783: f64, t13786: f64, t13789: f64, t13792: f64, t13993: f64, t13809: f64) -> (f64, f64) {
    let t13998 = 4.0_f64 / 27.0_f64 * t13794;
    let t13999 = -t13764 / 4.0_f64 + 3.0_f64 / 8.0_f64 * t13768 - t13772 / 2.0_f64 + t13775 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t13778 - t13993 + 2.0_f64 / 3.0_f64 * t13783 - 4.0_f64 / 3.0_f64 * t13786 + 2.0_f64 / 3.0_f64 * t13789 - 8.0_f64 / 3.0_f64 * t13792 + t13998;
    let t14004 = 2.0_f64 / 9.0_f64 * t13809;
    (t13999, t14004)
}
