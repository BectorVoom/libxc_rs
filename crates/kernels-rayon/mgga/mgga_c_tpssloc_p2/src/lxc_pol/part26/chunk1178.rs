//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1178/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1178(t24802: f64, t24866: f64, t1241: f64, t2144: f64, t3481: f64, t1190: f64, t7348: f64, t2154: f64, t3630: f64, t3598: f64, t225: f64, t7349: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t24867 = t24802 + t24866;
    let t24868 = t1241 * t24867;
    let t24871 = t3481 * t2144;
    let t24873 = t1190 * t7348;
    let t24876 = t2154 * t3630;
    let t24877 = t3598 * t24876;
    let t24880 = t7349 * t225;
    (t24867, t24868, t24871, t24873, t24877, t24880)
}
