//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 856/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk856(t10418: f64, t2389: f64, t34506: f64, t34507: f64, t41726: f64, t12766: f64, t1572: f64, t4673: f64, t41822: f64, t475: f64, t10340: f64, t1445: f64, t1562: f64, t2293: f64) -> (f64, f64, f64, f64, f64) {
    let t42001 = t10418 * t2389;
    let t42005 = 0.85801175884441024004e1_f64 * t34506 * t34507 * t41726;
    let t42008 = 0.47667319935800568892e0_f64 * t1572 * t4673 * t12766;
    let t42009 = t41822 * t475;
    let t42015 = t1562 * t1445 * t10340 * t2293;
    (t42001, t42005, t42008, t42009, t42015)
}
