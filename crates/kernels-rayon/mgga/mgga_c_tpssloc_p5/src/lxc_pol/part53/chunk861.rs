//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 861/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk861(t232: f64, t2646: f64, t4180: f64, t30714: f64, t235: f64, t835: f64, t226: f64, t8344: f64, t8343: f64, t849: f64, t8301: f64, t9231: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30716 = t4180 * t2646 * t232;
    let t30717 = t30714 * t30716;
    let t30719 = t235 * t835;
    let t30720 = t226 * t30719;
    let t30721 = t30720 * t8344;
    let t30723 = t8343 * t849;
    let t31000 = t9231 * t8301;
    (t30716, t30717, t30719, t30720, t30721, t30723, t31000)
}
