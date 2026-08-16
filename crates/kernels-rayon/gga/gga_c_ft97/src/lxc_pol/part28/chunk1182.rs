//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1182/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1182(t23405: f64, t34975: f64, t1349: f64, t35015: f64, t376: f64, t35234: f64, t1389: f64, t139600: f64, t147993: f64, t148943: f64, t1557: f64, t1570: f64, t1642: f64, t26823: f64, t27417: f64, t27420: f64, t27426: f64, t27428: f64, t3188: f64, t32714: f64, t32743: f64, t35028: f64, t5766: f64, t5772: f64, t6580: f64, t7313: f64) -> f64 {
    let t149460 = t23405 * t34975;
    let t149479 = t1349 * t376 * t35015;
    let t149483 = t23405 * t35234;
    let t149491 = t139600 / 9.0_f64 + t149460 / 54.0_f64 - t5766 * t35028 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t5772 * t27420 * t1389 * t1570 * t3188 - 2.0_f64 / 27.0_f64 * t5772 * t27426 * t1389 * t1557 * t3188 - t6580 * t32743 / 3.0_f64 + 4.0_f64 * t147993 + 4.0_f64 * t148943 + t149479 / 9.0_f64 - t32714 * t26823 / 18.0_f64 + t149483 / 27.0_f64 + 2.0_f64 / 27.0_f64 * t5772 * t1642 * t7313 * t27428 + t32714 * t27417 / 9.0_f64;
    t149491
}
