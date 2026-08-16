//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 702/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk702(t1039: f64, t590: f64, t574: f64, t5900: f64, t27157: f64, t23649: f64, t6662: f64, t2: f64, t6615: f64, t1969: f64, t379: f64, t5899: f64) -> (f64, f64, f64, f64) {
    let t27158 = t1039 * t590;
    let t27160 = t574 * t5900 * t27158;
    let t27161 = t27157 * t27160;
    let t27163 = t23649 * t6662;
    let t27165 = t2 * t6615;
    let t27167 = t1969 * t27165 * t379;
    let t27168 = t5899 * t27167;
    (t27158, t27161, t27163, t27168)
}
