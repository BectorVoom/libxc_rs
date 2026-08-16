//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1180/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1180(t265: f64, t504: f64, t24629: f64, t24900: f64, t3640: f64, t7394: f64, t11947: f64, t2157: f64, t1254: f64, t1256: f64, t193: f64, t23772: f64, t336: f64, t3633: f64, t3637: f64, t4700: f64, t7398: f64) -> (f64, f64, f64, f64) {
    let t505 = t265 < t504;
    let t24901 = t24629 + t24900;
    let t24905 = t7394 * t3640;
    let t24909 = t2157 * t11947;
    let t24916 = piecewise3(t505, t1256 * t193 * t24901 * t336 - 2.0_f64 * t1254 * t24905 * t4700 + 2.0_f64 * t24909 * t3637 * t4700 - t3633 * t4700 * t7398, t23772);
    (t24901, t24905, t24909, t24916)
}
