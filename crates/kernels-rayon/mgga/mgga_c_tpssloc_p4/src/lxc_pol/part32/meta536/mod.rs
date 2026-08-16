//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta536 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1874;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1875;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta536(t5075: f64, t7376: f64, t7375: f64, t225: f64, t8034: f64, t7364: f64, t5072: f64, t1215: f64, t1409: f64, t24851: f64, t24589: f64, t24812: f64, t24827: f64, t24849: f64, t27406: f64, t27481: f64, t27484: f64, t27492: f64, t27498: f64, t27502: f64, t27507: f64, t7283: f64, t7368: f64, t7373: f64, t7378: f64, t1755: f64, t7327: f64, t1090: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t27510, t27511, t27516, t27517, t27520, t27521, t27525, t27526, t27529) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1874(t5075, t7376, t7375, t225, t8034, t7364, t5072, t1215, t1409, t24851, t24589, t24812, t24827, t24849, t27406, t27481, t27484, t27492, t27498, t27502, t27507, t7283, t7368, t7373, t7378);
        let (t27532, t27533, t27536) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1875(t1755, t7327, t1090, t7376, t8034);
    (t27510, t27511, t27516, t27517, t27520, t27521, t27525, t27526, t27529, t27532, t27533, t27536)
}
