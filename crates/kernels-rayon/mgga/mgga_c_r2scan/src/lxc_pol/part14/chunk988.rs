//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 988/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk988(t11670: f64, t11671: f64, t10760: f64, t7619: f64, t6093: f64, t7624: f64, t2147: f64, t3344: f64, t980: f64, t8089: f64, t6535: f64, t261: f64, t2726: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11672 = t11670 * t11671;
    let t11675 = t10760 * t7619;
    let t11676 = t6093 * t11675;
    let t11678 = t10760 * t7624;
    let t11679 = t2147 * t11678;
    let t11681 = t980 * t3344;
    let t11683 = t10760 * t8089;
    let t11684 = t6535 * t11683;
    let t11686 = t261 * t2726;
    (t11672, t11675, t11676, t11678, t11679, t11681, t11683, t11684, t11686)
}
