//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1068/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1068(t5514: f64, t935: f64, t1858: f64, t2530: f64, t2021: f64, t7530: f64, t1854: f64, t20901: f64, t5679: f64, t6110: f64, t5580: f64, t7426: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23292 = t5514 * t935;
    let t23296 = t1858 * t2530;
    let t23309 = t2021 * t7530;
    let t23310 = t20901 * t1854;
    let t23335 = t5679 * t6110;
    let t23344 = t5580 * t7426;
    (t23292, t23296, t23309, t23310, t23335, t23344)
}
