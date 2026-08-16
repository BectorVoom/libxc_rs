//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1099/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1099(t28563: f64, t2586: f64, t2617: f64, t7803: f64, t7344: f64, t948: f64, t20671: f64, t22543: f64, t22980: f64, t21461: f64, t2365: f64, t7390: f64) -> (f64, f64, f64, f64, f64) {
    let t28564 = 0.76685851907841499352e0_f64 * t28563;
    let t28566 = t7803 * t2586 * t2617;
    let t28567 = 0.76685851907841499352e0_f64 * t28566;
    let t28569 = t7803 * t948 * t7344;
    let t28570 = 0.38342925953920749676e0_f64 * t28569;
    let t28585 = 0.17041300423964777634e0_f64 * t22543 * t20671 * t22980;
    let t28593 = 0.29792074959875355558e-1_f64 * t7390 * t2365 * t21461;
    (t28564, t28567, t28570, t28585, t28593)
}
