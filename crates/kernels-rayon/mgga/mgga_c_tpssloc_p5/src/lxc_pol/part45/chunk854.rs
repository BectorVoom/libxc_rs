//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 854/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk854(t30714: f64, t30716: f64, t235: f64, t835: f64, t226: f64, t8344: f64, t8343: f64, t849: f64, t6547: f64, t8336: f64, t25: f64, t6665: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30717 = t30714 * t30716;
    let t30719 = t235 * t835;
    let t30720 = t226 * t30719;
    let t30721 = t30720 * t8344;
    let t30723 = t8343 * t849;
    let t30748 = 0.38381794893125283518e-1_f64 * t6547 * t8336;
    let t30767 = t25 * t6665;
    (t30717, t30719, t30720, t30721, t30723, t30748, t30767)
}
