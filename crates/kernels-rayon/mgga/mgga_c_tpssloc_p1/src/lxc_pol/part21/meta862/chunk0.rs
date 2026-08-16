//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3129/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3129(t16558: f64, t3450: f64, t11588: f64, t6138: f64, t3447: f64, t3451: f64, t4904: f64, t52036: f64, t15313: f64, t15338: f64, t18523: f64, t3448: f64) -> (f64, f64, f64, f64, f64) {
    let t64756 = t3450 * t16558;
    let t64763 = t11588 * t6138;
    let t64765 = t3447 * t64763 * t3451;
    let t64770 = t3447 * t52036 * t4904;
    let t64773 = t3447 * t15338 * t15313;
    let t64775 = t3448 * t18523;
    (t64756, t64765, t64770, t64773, t64775)
}
