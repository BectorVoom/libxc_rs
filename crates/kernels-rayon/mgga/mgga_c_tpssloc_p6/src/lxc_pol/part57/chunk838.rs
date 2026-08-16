//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 838/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk838(t30713: f64, t812: f64, t235: f64, t835: f64, t226: f64, t8344: f64, t6547: f64, t8336: f64, t2015: f64, t3886: f64, t1377: f64, t794: f64, t8454: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30714 = t812 * t30713;
    let t30719 = t235 * t835;
    let t30720 = t226 * t30719;
    let t30721 = t30720 * t8344;
    let t30748 = 0.38381794893125283518e-1_f64 * t6547 * t8336;
    let t31090 = t3886 * t2015;
    let t31099 = t1377 * t2015;
    let t31104 = t794 * t8454;
    (t30714, t30719, t30720, t30721, t30748, t31090, t31099, t31104)
}
