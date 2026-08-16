//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1765/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1765(t13487: f64, t22960: f64, t606: f64, t776: f64, t25: f64, t2553: f64, t1911: f64, t2742: f64, t2718: f64, t6662: f64, t865: f64, t2684: f64, t6657: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22961 = t22960 * t13487;
    let t22964 = t606 * t776;
    let t22968 = t25 * t2553;
    let t22974 = t1911 * t2742;
    let t22975 = t2718 * t22974;
    let t22978 = t6662 * t865;
    let t22979 = t2718 * t22978;
    let t22984 = t6657 * t2684;
    (t22961, t22964, t22968, t22975, t22979, t22984)
}
