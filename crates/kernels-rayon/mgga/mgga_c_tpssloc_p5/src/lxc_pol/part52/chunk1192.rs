//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1192/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1192(t31840: f64, t31849: f64, t31895: f64, t31921: f64, t3: f64, t112: f64, t8692: f64, t1873: f64, t24969: f64, t24972: f64, t7015: f64, t6534: f64, t7423: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31923 = t31840 + t31849 + t31895 + t31921;
    let t31924 = t3 * t31923;
    let t31937 = t8692 * t112;
    let t31940 = t24969 * t1873;
    let t31942 = t24972 * t7015;
    let t31944 = t7423 * t6534;
    (t31923, t31924, t31937, t31940, t31942, t31944)
}
