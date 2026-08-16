//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta168 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk895;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk896;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk897;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta168(t25: f64, t1268: f64, t2312: f64, t2314: f64, t2319: f64, t2363: f64, t671: f64, t88: f64, t526: f64, t606: f64, t2249: f64, t514: f64, t528: f64, zeta_threshold: f64, t28: f64, t1081: f64, t3231: f64, t517: f64, t157: f64, t182: f64, t118: f64, t521: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3660, t3664, t3665, t3671, t3672) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk895(t25, t1268, t2312, t2314, t2319, t2363, t671, t88, t526, t606, t2249, t514, t528, zeta_threshold);
        let (t3673, t3681) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk896(t28, t1081, t3231, t3672, t517, t157, t3671, zeta_threshold);
        let (t3683, t3684) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk897(t182, t3681, t118, t521);
    (t3660, t3664, t3665, t3672, t3673, t3681, t3683, t3684)
}
