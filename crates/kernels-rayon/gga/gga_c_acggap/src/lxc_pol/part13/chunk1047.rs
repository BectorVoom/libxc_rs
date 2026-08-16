//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1047/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1047(t1511: f64, t2020: f64, t31146: f64, t4487: f64, t7815: f64, t2030: f64, t5160: f64, t7440: f64, t8631: f64, t2318: f64, t31261: f64, t7538: f64, t8689: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34382 = t2020 * t1511;
    let t34383 = 7.0_f64 / 144.0_f64 * t34382;
    let t34385 = t31146 * t7815 * t4487;
    let t34388 = t2030 * t7815 * t5160;
    let t34390 = t7440 * t8631;
    let t34391 = 0.5603125e-1_f64 * t34390;
    let t34392 = t31261 * t2318;
    let t34394 = t7538 * t8689;
    (t34383, t34385, t34388, t34391, t34392, t34394)
}
