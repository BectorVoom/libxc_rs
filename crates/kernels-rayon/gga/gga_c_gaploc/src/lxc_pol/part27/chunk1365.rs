//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1365/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1365(t105: f64, t1083: f64, t11978: f64, t11988: f64, t12019: f64, t1324: f64, t169: f64, t172: f64, t29892: f64, t31570: f64, t31575: f64, t31577: f64, t31581: f64, t31584: f64, t31589: f64, t31594: f64, t31600: f64, t3692: f64, t3701: f64, t380: f64, t3818: f64, t3822: f64, t38313: f64, t452: f64, t6313: f64) -> f64 {
    let t38337 = 0.7588001769513639893e-1_f64 * t380 * t12019 + 0.7588001769513639893e-1_f64 * t1083 * t3692 + t29892 + t31570 + t31575 + 0.7588001769513639893e-1_f64 * t3818 * t11988 + 0.56910013271352299198e-1_f64 * t3822 * t3701 * t1324 - 0.2276400530854091968e0_f64 * t6313 * t11978 + t31577 + t31581 + t31584 - t31589 - t31594 - t31600 + 0.28455006635676149599e-1_f64 * t105 * t452 * t38313 * t169 * t172;
    t38337
}
