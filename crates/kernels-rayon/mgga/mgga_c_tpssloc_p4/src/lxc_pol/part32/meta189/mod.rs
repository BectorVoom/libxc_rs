//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta189 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk922;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk923;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk924;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk925;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk926;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta189(t466: f64, t5052: f64, t1752: f64, t225: f64, t1251: f64, t1760: f64, t3598: f64, t1243: f64, t5000: f64, t1215: f64, t3612: f64, t1755: f64, t1235: f64, t1734: f64, t1246: f64, t491: f64, t5011: f64, t1932: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5053, t5055) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk922(t466, t5052, t1752, t225);
        let t5059 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk923(t1251, t1760);
        let t5060 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk924(t3598, t5059);
        let t5064 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk925(t1243, t5000);
        let t5068 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk926(t1215, t3612);
        let (t5069, t5072, t5073, t5075, t5076, t5079) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk927(t1755, t5068, t1235, t1734, t1246, t491, t5011, t1215, t1932, t475);
    (t5053, t5055, t5059, t5060, t5064, t5068, t5069, t5072, t5073, t5075, t5076, t5079)
}
