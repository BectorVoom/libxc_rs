//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1320/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1320(t56955: f64, t57229: f64, t57521: f64, t57528: f64, t25093: f64, t55901: f64, t894: f64, t2596: f64, t55906: f64, t25001: f64, t4776: f64, t8201: f64) -> (f64, f64, f64, f64, f64) {
    let t57530 = t56955 + t57229 + t57521 + t57528;
    let t57537 = t894 * t25093 * t55901;
    let t57541 = t894 * t2596 * t55906;
    let t57545 = t894 * t25001 * t55901;
    let t57554 = t8201 * t4776;
    (t57530, t57537, t57541, t57545, t57554)
}
