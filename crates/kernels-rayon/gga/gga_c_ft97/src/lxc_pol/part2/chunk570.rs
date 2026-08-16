//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 570/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk570(t200: f64, t3761: f64, t1100: f64, t226: f64, t1113: f64, t694: f64, t709: f64, t677: f64, t236: f64, t6: f64, t51: f64, t1614: f64, t213: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3762 = t3761 * t200;
    let t3766 = t1100 * t226;
    let t3767 = t694 * t1113;
    let t3768 = t3767 * t709;
    let t3771 = t677 * t226;
    let t3772 = t236 * t6;
    let t3773 = t3772 * t51;
    let t3774 = t3771 * t3773;
    let t3775 = t1614 * t213;
    (t3762, t3766, t3767, t3768, t3771, t3774, t3775)
}
