//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 982/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk982(t1937: f64, t33602: f64, t6985: f64, t7735: f64, t13272: f64, t8435: f64, t1497: f64, t8441: f64, t8621: f64, t1469: f64, t32143: f64, t7714: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33603 = t33602 * t1937;
    let t33605 = t6985 * t7735;
    let t33609 = t13272 * t8435;
    let t33612 = t8441 * t1497;
    let t33613 = t8621 * t33612;
    let t33617 = t8621 * t32143 * t1469;
    let t33620 = t8441 * t7714;
    (t33603, t33605, t33609, t33612, t33613, t33617, t33620)
}
