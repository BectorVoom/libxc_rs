//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1260/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1260(t1416: f64, t3116: f64, t9180: f64, t13853: f64, t169: f64, t21204: f64, t4043: f64, t519: f64, t11430: f64, t3060: f64, t8716: f64, t11350: f64, t9241: f64) -> (f64, f64, f64, f64) {
    let t35019 = t9180 * t1416 * t3116;
    let t35024 = t169 * t21204 * t4043 * t519 * t13853;
    let t35027 = t3060 * t11430 * t8716;
    let t35031 = t11350 * t9241;
    (t35019, t35024, t35027, t35031)
}
