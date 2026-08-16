//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1176/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1176(t1170: f64, t8077: f64, t2121: f64, t1751: f64, t7299: f64, t24574: f64, t8015: f64, t8006: f64, t3242: f64, t497: f64, t254: f64, t492: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27736 = t1170 * t8077;
    let t27737 = t2121 * t27736;
    let t27751 = t7299 * t1751;
    let t27755 = t24574 * t8015;
    let t27770 = t24574 * t8006;
    let t27774 = t497 * t3242;
    let t27784 = t492 * t254;
    (t27736, t27737, t27751, t27755, t27770, t27774, t27784)
}
