//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta194 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk941;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk942;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta194(t1804: f64, t3726: f64, t131: f64, t3732: f64, t205: f64, t1799: f64, t213: f64, t1307: f64, t221: f64, t118: f64, t794: f64, t3739: f64, t210: f64, t214: f64, t5187: f64, t1315: f64, t3725: f64, t3727: f64, t3731: f64, t3742: f64, t3751: f64, t562: f64, t1372: f64, t1807: f64, t1808: f64, t225: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5192, t5194, t5195, t5196, t5198, t5202, t5203) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk941(t1804, t3726, t131, t3732, t205, t1799, t213, t1307, t221, t118, t794, t3739);
        let (t5206, t5210) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk942(t210, t214, t5187, t1315, t3725, t3727, t3731, t3742, t3751, t5192, t5195, t5198, t5203);
        let (t5211, t5213, t5215) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk943(t5210, t562, t1372, t1807, t1808, t225);
    (t5192, t5194, t5195, t5196, t5198, t5202, t5203, t5206, t5210, t5211, t5213, t5215)
}
