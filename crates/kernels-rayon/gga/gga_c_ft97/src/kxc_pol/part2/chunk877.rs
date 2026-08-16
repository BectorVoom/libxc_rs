//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 877/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk877(t13623: f64, t13650: f64, t1095: f64, t694: f64, t3724: f64, t709: f64, t13395: f64, t3785: f64, t13434: f64, t9609: f64, t1096: f64, t2428: f64, t680: f64) -> (f64, f64, f64, f64, f64) {
    let t13651 = t13623 + t13650;
    let t13654 = t694 * t1095;
    let t13656 = t3724 * t13654 * t709;
    let t13659 = t3785 * t13395;
    let t13662 = t9609 * t13434;
    let t13666 = t680 * t1096 * t2428;
    (t13651, t13656, t13659, t13662, t13666)
}
