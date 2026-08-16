//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 847/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk847(t34506: f64, t34507: f64, t41726: f64, t12766: f64, t1572: f64, t4673: f64, t12919: f64, t4953: f64, t1445: f64, t1562: f64, t3116: f64, t8097: f64) -> (f64, f64, f64, f64) {
    let t42005 = 0.85801175884441024004e1_f64 * t34506 * t34507 * t41726;
    let t42008 = 0.47667319935800568892e0_f64 * t1572 * t4673 * t12766;
    let t42018 = 0.69017266717057349418e1_f64 * t4953 * t12919;
    let t42022 = 0.69017266717057349418e1_f64 * t1562 * t1445 * t8097 * t3116;
    (t42005, t42008, t42018, t42022)
}
