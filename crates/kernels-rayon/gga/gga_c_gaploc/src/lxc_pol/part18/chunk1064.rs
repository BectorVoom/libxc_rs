//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1064/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1064(t1984: f64, t9804: f64, t5501: f64, t935: f64, t2530: f64, t321: f64, t5580: f64, t7802: f64, t7809: f64, t2012: f64, t7426: f64, t1423: f64, t2554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23000 = t1984 * t9804;
    let t23021 = t5501 * t935;
    let t23092 = t321 * t2530;
    let t23099 = t5580 * t7802;
    let t23104 = t5580 * t7809;
    let t23157 = t2012 * t7426;
    let t23176 = t1423 * t2554;
    (t23000, t23021, t23092, t23099, t23104, t23157, t23176)
}
