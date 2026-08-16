//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1223/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1223(t10694: f64, t1841: f64, t10698: f64, t29439: f64, t5398: f64, t7064: f64, t8878: f64, t10629: f64, t5524: f64, t1897: f64, t27661: f64, t954: f64) -> (f64, f64, f64, f64, f64) {
    let t32668 = t1841 * t10694;
    let t32669 = 0.17090058289204942853e-2_f64 * t32668;
    let t32670 = t29439 * t10698;
    let t32671 = 0.19226315575355560709e-2_f64 * t32670;
    let t32673 = t7064 * t8878 * t5398;
    let t32674 = 0.1922631557535556071e-2_f64 * t32673;
    let t32676 = 0.17090058289204942851e-2_f64 * t5524 * t10629;
    let t32679 = 0.15381052460284448567e-1_f64 * t1897 * t954 * t27661;
    (t32669, t32671, t32674, t32676, t32679)
}
