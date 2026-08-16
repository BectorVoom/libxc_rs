//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 906/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk906(t1100: f64, t13463: f64, t3781: f64, t7853: f64, t5025: f64, t9681: f64, t1109: f64, t1127: f64, t709: f64, t14722: f64, t4978: f64, t680: f64) -> (f64, f64, f64, f64, f64) {
    let t17993 = t1100 * t13463;
    let t17994 = t7853 * t3781;
    let t17997 = t9681 * t5025;
    let t18001 = t1109 * t1127;
    let t18002 = t18001 * t709;
    let t18003 = t14722 * t18002;
    let t18006 = t4978 * t709;
    let t18007 = t680 * t18006;
    (t17993, t17994, t17997, t18003, t18007)
}
