//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 146/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk146(t492: f64, t493: f64, t186: f64, t137: f64, t1: f64, t124: f64, t3: f64, t4: f64, t487: f64, t141: f64, t483: f64, t486: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t494 = t492 * t493;
    let t495 = 1.0_f64 / t186;
    let t496 = t137 * t495;
    let t498 = t124 * t1 * t3;
    let t499 = t496 * t498;
    let t502 = t487 * t4;
    let t505 = -0.19415017735199121314e-1_f64 * t483 * t141 - 0.24268772168998901643e-2_f64 * t486 * t488 + 0.24268772168998901643e-3_f64 * t494 * t499 - 0.43149876916480047122e-3_f64 * t494 * t502;
    (t494, t495, t496, t498, t499, t502, t505)
}
