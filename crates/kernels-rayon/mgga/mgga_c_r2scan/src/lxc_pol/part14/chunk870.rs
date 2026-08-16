//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 870/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk870(t2055: f64, t2056: f64, t955: f64, t2768: f64, t761: f64, t2061: f64, t6030: f64, t6033: f64, t7108: f64, t7110: f64, t7112: f64, t7126: f64, t7128: f64, t7149: f64, t7150: f64) -> f64 {
    let t7898 = t2055 * t955 * t2056;
    let t7902 = t2768 * t761;
    let t7904 = 0.1350520664e0_f64 * t2061 * t7902;
    let t7905 = t7108 - t7110 - t7112 - 0.571528e-1_f64 * t7898 + 0.2701041328e0_f64 * t6030 - 0.675260332e-1_f64 * t6033 - t7126 - t7128 - t7904 + t7149 + t7150;
    t7905
}
