//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1188/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1188(t20171: f64, t33287: f64, t5708: f64, t19533: f64, t19535: f64, t11587: f64, t11591: f64, t3060: f64, t28006: f64, t3112: f64, t33498: f64, t8362: f64) -> (f64, f64, f64, f64) {
    let t34764 = t5708 * t33287 * t20171;
    let t34767 = t19533 * t33287 * t19535;
    let t34772 = t3060 * t11587 * t11591;
    let t34776 = t3112 * t33498 * t8362 * t28006;
    (t34764, t34767, t34772, t34776)
}
