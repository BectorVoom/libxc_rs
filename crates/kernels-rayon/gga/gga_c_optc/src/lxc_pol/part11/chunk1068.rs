//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1068/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1068(t1431: f64, t7330: f64, t1587: f64, t23471: f64, t429: f64, t1585: f64, t1443: f64, t3649: f64) -> (f64, f64, f64) {
    let t33596 = t1431 * t7330;
    let t33683 = t23471 * t429 * t1587;
    let t33684 = t1585 * t33683;
    let t33724 = t3649 * t1443;
    (t33596, t33684, t33724)
}
