//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1196/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1196(t11503: f64, t8916: f64, t34793: f64, t8684: f64, t11526: f64, t26609: f64, t129: f64, t21655: f64, t21657: f64, t3021: f64, t11474: f64, t8880: f64) -> (f64, f64, f64, f64, f64) {
    let t34822 = t8916 * t11503;
    let t34824 = t8684 * t34793;
    let t34826 = t11526 * t26609;
    let t34830 = t21655 * t129 * t3021 * t21657;
    let t34832 = t11474 * t8880;
    (t34822, t34824, t34826, t34830, t34832)
}
