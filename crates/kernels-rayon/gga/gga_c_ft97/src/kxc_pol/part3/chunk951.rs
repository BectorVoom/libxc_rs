//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 951/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk951(t18690: f64, t3891: f64, t14098: f64, t18514: f64, t14081: f64, t14080: f64, t18497: f64, t3892: f64, t11593: f64, t18643: f64, t18648: f64, t18652: f64, t18656: f64, t18660: f64, t18664: f64, t18668: f64, t18672: f64, t18677: f64, t18682: f64, t18687: f64, t1901: f64, t3281: f64, t446: f64) -> f64 {
    let t18691 = t3891 * t18690;
    let t18694 = t14098 * t18514;
    let t18695 = t3891 * t18694;
    let t18698 = t14081 * t18514;
    let t18699 = t14080 * t18698;
    let t18702 = t3892 * t18497;
    let t18703 = t3891 * t18702;
    let t18706 = 2.0_f64 / 3.0_f64 * t446 * t18643 + t446 * t18648 / 3.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t18652 + 4.0_f64 / 3.0_f64 * t446 * t18656 + 4.0_f64 / 3.0_f64 * t446 * t18660 + 2.0_f64 / 3.0_f64 * t446 * t18664 + 4.0_f64 / 9.0_f64 * t3281 * t18668 - 4.0_f64 / 3.0_f64 * t1901 * t18672 - 4.0_f64 / 3.0_f64 * t1901 * t18677 - 2.0_f64 / 9.0_f64 * t1901 * t18682 - 2.0_f64 / 9.0_f64 * t1901 * t18687 + 2.0_f64 / 27.0_f64 * t1901 * t18691 + 4.0_f64 / 9.0_f64 * t1901 * t18695 - 10.0_f64 / 81.0_f64 * t1901 * t18699 + 8.0_f64 / 27.0_f64 * t11593 * t18703;
    t18706
}
