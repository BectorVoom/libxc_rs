//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta473 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1771;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1772;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta473(t1089: f64, t491: f64, t7327: f64, t24574: f64, t7365: f64, t1235: f64, t477: f64, t225: f64, t7349: f64, t7288: f64, t7306: f64, t3640: f64, t7394: f64, t11947: f64, t2157: f64, t111: f64, t7263: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24850, t24851, t24856, t24858, t24880, t24891, t24893, t24905) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1771(t1089, t491, t7327, t24574, t7365, t1235, t477, t225, t7349, t7288, t7306, t3640, t7394);
        let (t24909, t24932) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1772(t11947, t2157, t111, t7263);
    (t24850, t24851, t24856, t24858, t24880, t24891, t24893, t24905, t24909, t24932)
}
