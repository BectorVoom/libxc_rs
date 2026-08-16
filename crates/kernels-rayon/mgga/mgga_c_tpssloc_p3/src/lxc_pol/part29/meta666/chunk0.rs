//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2219/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2219(t90963: f64, t22633: f64, t26421: f64, t3851: f64, t6976: f64, t26418: f64, t6914: f64, t7736: f64, t80854: f64, t81064: f64, t22704: f64, t22705: f64, t26410: f64) -> (f64, f64, f64, f64, f64) {
    let t90964 = 0.76763589786250567036e-1_f64 * t90963;
    let t90968 = t22633 * t6976 * t26421 * t3851;
    let t90970 = t6914 * t26418;
    let t90971 = 0.38381794893125283518e-1_f64 * t90970;
    let t90980 = t81064 * t80854 * t7736;
    let t90983 = t22704 * t22705 * t26410;
    (t90964, t90968, t90971, t90980, t90983)
}
