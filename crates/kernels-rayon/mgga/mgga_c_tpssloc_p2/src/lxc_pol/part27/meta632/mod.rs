//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta632 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2129;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2130;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta632(t22986: f64, t25192: f64, t82159: f64, t254: f64, t853: f64, t23164: f64, t23204: f64, t25341: f64, t12971: f64, t6552: f64, t6553: f64, t6554: f64, t776: f64, t865: f64, t23270: f64, t25044: f64, t82147: f64, t13377: f64, t1880: f64, t214: f64, t225: f64, t258: f64, t1887: f64, t81956: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t87010, t87013, t87029, t87033) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2129(t22986, t25192, t82159, t254, t853, t23164, t23204, t25341, t12971, t6552, t6553, t6554);
        let (t87036, t87039, t87042, t87047, t87049) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2130(t776, t865, t22986, t23270, t25044, t82147, t13377, t1880, t214, t225, t258, t1887, t81956);
    (t87010, t87013, t87029, t87033, t87036, t87039, t87042, t87047, t87049)
}
