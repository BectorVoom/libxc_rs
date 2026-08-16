//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1333/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1333(t1617: f64, t3822: f64, t4915: f64, t12052: f64, t23726: f64, t2011: f64, t3808: f64, t30472: f64, t3483: f64, t12038: f64, t575: f64, t687: f64) -> (f64, f64, f64, f64, f64) {
    let t36098 = 6.0_f64 * t4915 * t3822 * t1617;
    let t36100 = 12.0_f64 * t23726 * t12052;
    let t36103 = 6.0_f64 * t4915 * t3808 * t2011;
    let t36105 = 4.0_f64 * t30472 * t3483;
    let t36106 = t12038 * t575;
    let t36108 = 2.0_f64 * t36106 * t687;
    (t36098, t36100, t36103, t36105, t36108)
}
