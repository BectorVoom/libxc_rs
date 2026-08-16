//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 831/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk831(t13054: f64, t2210: f64, t12603: f64, t144: f64, t1882: f64, t3567: f64, t1017: f64, t2180: f64, t2179: f64, t574: f64, t1986: f64, t167: f64, t9432: f64) -> (f64, f64, f64, f64, f64) {
    let t13055 = t2210 * t13054;
    let t13058 = t144 * t12603;
    let t13062 = 2.0_f64 / 9.0_f64 * t1882 * t3567;
    let t13065 = t1017 * t2180;
    let t13067 = t574 * t2179 * t13065;
    let t13070 = t1017 * t1986;
    let t13072 = t9432 * t167 * t13070;
    (t13055, t13058, t13062, t13067, t13072)
}
