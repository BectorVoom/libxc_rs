//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1123/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1123(t14656: f64, t795: f64, t3270: f64, t10966: f64, t1103: f64, t269: f64, t955: f64, t11505: f64, t494: f64, t97: f64, t3446: f64, t37475: f64, t970: f64) -> (f64, f64, f64, f64) {
    let t40648 = t14656 * t795;
    let t40649 = t3270 * t40648;
    let t40659 = t10966 * t1103 * t955 * t269;
    let t40664 = t97 * t11505 * t494;
    let t40672 = t3446 * t37475 * t970;
    (t40649, t40659, t40664, t40672)
}
