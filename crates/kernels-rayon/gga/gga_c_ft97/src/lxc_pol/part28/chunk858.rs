//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 858/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk858(t1909: f64, t34677: f64, t7229: f64, t925: f64, t8557: f64, t32494: f64, t8217: f64, t32515: f64, t1901: f64, t32487: f64, t32508: f64, t32510: f64, t32587: f64, t34663: f64, t34667: f64, t34671: f64, t34674: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34678 = t1909 * t34677;
    let t34681 = t7229 * t925;
    let t34682 = t8557 * t34681;
    let t34685 = t32494 * t925;
    let t34686 = t8217 * t34685;
    let t34689 = t32515 * t925;
    let t34690 = t1909 * t34689;
    let t34693 = 2.0_f64 / 3.0_f64 * t446 * t34663 + 2.0_f64 / 3.0_f64 * t446 * t34667 - t32487 - t32508 + t32510 - 4.0_f64 / 3.0_f64 * t1901 * t34671 + 2.0_f64 / 9.0_f64 * t1901 * t34674 - 2.0_f64 / 9.0_f64 * t1901 * t34678 - 2.0_f64 / 9.0_f64 * t1901 * t34682 - 2.0_f64 / 9.0_f64 * t1901 * t34686 + t1901 * t34690 / 9.0_f64 - t32587;
    (t34678, t34681, t34682, t34685, t34686, t34689, t34690, t34693)
}
