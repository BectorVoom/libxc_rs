//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 913/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk913(t11311: f64, t1875: f64, t1036: f64, t3949: f64, t611: f64, t3954: f64, t640: f64, t644: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11312 = t1875 * t11311;
    let t11313 = t1036 * t3949;
    let t11314 = t11312 * t11313;
    let t11316 = t611 * t11311;
    let t11317 = t1036 * t3954;
    let t11318 = t11316 * t11317;
    let t11320 = t640 * t644;
    (t11312, t11313, t11314, t11316, t11317, t11318, t11320)
}
