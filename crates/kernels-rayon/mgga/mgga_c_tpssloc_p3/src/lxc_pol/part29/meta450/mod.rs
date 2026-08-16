//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta450 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1764;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1765;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta450(t25: f64, t2752: f64, t13487: f64, t606: f64, t776: f64, t2553: f64, t1911: f64, t2742: f64, t2718: f64, t6662: f64, t865: f64, t2684: f64, t6657: f64, t1887: f64, t6581: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let t22960 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1764(t25, t2752);
        let (t22961, t22964, t22968, t22975, t22979, t22984) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1765(t13487, t22960, t606, t776, t25, t2553, t1911, t2742, t2718, t6662, t865, t2684, t6657);
        let t22986 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1766(t1887, t6581);
    (t22960, t22961, t22964, t22968, t22975, t22979, t22984, t22986)
}
