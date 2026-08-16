//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 757/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk757(t2121: f64, t27736: f64, t1751: f64, t7299: f64, t24574: f64, t8015: f64, t8006: f64, t225: f64, t8055: f64, t7280: f64, t7999: f64, t1170: f64, t8010: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27737 = t2121 * t27736;
    let t27751 = t7299 * t1751;
    let t27755 = t24574 * t8015;
    let t27770 = t24574 * t8006;
    let t27792 = t8055 * t225;
    let t27808 = t7999 * t7280;
    let t27817 = t1170 * t8010;
    (t27737, t27751, t27755, t27770, t27792, t27808, t27817)
}
