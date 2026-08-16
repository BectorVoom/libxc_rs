//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 561/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk561(t1334: f64, t8232: f64, t1882: f64, t5745: f64, t5641: f64, t5650: f64, t1326: f64, t463: f64) -> (f64, f64, f64, f64, f64) {
    let t23311 = 4.0_f64 / 27.0_f64 * t8232 * t1334;
    let t23312 = t1882 * t5745;
    let t23319 = t1882 * t5641;
    let t23321 = t1882 * t5650;
    let t23323 = t463 * t1326;
    (t23311, t23312, t23319, t23321, t23323)
}
