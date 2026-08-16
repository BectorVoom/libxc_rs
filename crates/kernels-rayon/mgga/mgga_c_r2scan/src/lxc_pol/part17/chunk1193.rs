//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1193/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1193(t29451: f64, t37717: f64, t29454: f64, t37720: f64, t11824: f64, t2207: f64, t3613: f64, t12511: f64, t6205: f64, t2124: f64, t29496: f64, t39849: f64) -> (f64, f64, f64, f64, f64) {
    let t43643 = t37717 * t29451;
    let t43645 = t37720 * t29454;
    let t43648 = t2207 * t3613 * t11824;
    let t43650 = t6205 * t12511;
    let t43654 = t39849 * t2124 * t29496;
    (t43643, t43645, t43648, t43650, t43654)
}
