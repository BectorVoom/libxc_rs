//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1028/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1028(t446: f64, t85483: f64, t8577: f64, t7824: f64, t86094: f64, t1564: f64, t86075: f64, t15601: f64, t28: f64, t4495: f64, t89: f64, t1555: f64, t7764: f64, t85469: f64) -> (f64, f64, f64, f64, f64) {
    let t86220 = t446 * t8577 * t85483;
    let t86223 = t446 * t7824 * t86094;
    let t86226 = t446 * t1564 * t86075;
    let t86232 = t89 * t28 * t15601 * t4495;
    let t86236 = t89 * t1555 * t7764 * t85469;
    (t86220, t86223, t86226, t86232, t86236)
}
