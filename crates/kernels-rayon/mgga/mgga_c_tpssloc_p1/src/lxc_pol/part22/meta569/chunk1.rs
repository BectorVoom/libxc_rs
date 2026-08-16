//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2077/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2077(t221: f64, t339: f64, t42813: f64, t10216: f64, t2978: f64, t10479: f64, t42333: f64, t3061: f64, t676: f64, t11065: f64, t42387: f64, t1005: f64, t10375: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43307 = 5.0_f64 / 486.0_f64 * t339 * t221 * t42813;
    let t43317 = t2978 * t10216;
    let t43322 = t42333 * t10479;
    let t43338 = t676 * t3061;
    let t43361 = t11065 * t42387;
    let t43382 = t1005 * t10375;
    (t43307, t43317, t43322, t43338, t43361, t43382)
}
