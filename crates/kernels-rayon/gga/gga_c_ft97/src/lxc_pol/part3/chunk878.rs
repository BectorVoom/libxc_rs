//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 878/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk878(t11167: f64, t11177: f64, t12216: f64, t12217: f64, t16832: f64, t16842: f64, t16845: f64, t17666: f64, t3056: f64, t3359: f64, t383: f64, t7946: f64, t8698: f64) -> f64 {
    let t17667 = 0.1760655e0_f64 * t16832 * t383 - 0.234754e0_f64 * t3359 * t3056 - 0.117377e0_f64 * t16842 * t383 + 0.234754e0_f64 * t16845 - t8698 - 0.6419148148148148148e-1_f64 * t7946 - 0.12838296296296296296e0_f64 * t11167 + t12217 - t12216 + 0.19257444444444444444e0_f64 * t11177 + t17666;
    t17667
}
