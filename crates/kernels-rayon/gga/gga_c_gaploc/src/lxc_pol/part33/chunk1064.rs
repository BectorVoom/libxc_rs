//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1064/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1064(t10007: f64, t7068: f64, t10012: f64, t1984: f64, t9804: f64, t5501: f64, t935: f64, t2530: f64, t321: f64, t5580: f64, t7802: f64, t7809: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22980 = t10007 * t7068;
    let t22984 = t10012 * t7068;
    let t23000 = t1984 * t9804;
    let t23021 = t5501 * t935;
    let t23092 = t321 * t2530;
    let t23099 = t5580 * t7802;
    let t23104 = t5580 * t7809;
    (t22980, t22984, t23000, t23021, t23092, t23099, t23104)
}
