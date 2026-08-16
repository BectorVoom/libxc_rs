//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1244/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1244(t21497: f64, t32616: f64, t1897: f64, t29190: f64, t2936: f64, t10704: f64, t1850: f64, t10636: f64, t5227: f64, t1841: f64, t3487: f64, t7275: f64, t734: f64) -> (f64, f64, f64, f64, f64) {
    let t32618 = 0.34180116578409885704e-2_f64 * t21497 * t32616;
    let t32621 = 0.46143157380853345702e-1_f64 * t1897 * t2936 * t29190;
    let t32622 = t1850 * t10704;
    let t32623 = 0.85450291446024714264e-3_f64 * t32622;
    let t32625 = 0.17090058289204942853e-2_f64 * t5227 * t10636;
    let t32629 = 0.17090058289204942853e-2_f64 * t1841 * t7275 * t3487 * t734;
    (t32618, t32621, t32623, t32625, t32629)
}
