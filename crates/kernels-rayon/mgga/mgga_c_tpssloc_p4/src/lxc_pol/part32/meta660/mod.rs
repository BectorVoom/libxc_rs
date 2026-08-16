//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2089;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta660(t27654: f64, t491: f64, t1235: f64, t8034: f64, t27434: f64, t85639: f64, t27821: f64, t24600: f64, t7301: f64, t27798: f64, t4935: f64, t24615: f64, t24637: f64, t8009: f64, t24588: f64, t8020: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t94354, t94358, t94363, t94365, t94369, t94374, t94378) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2089(t27654, t491, t1235, t8034, t27434, t85639, t27821, t24600, t7301, t27798, t4935, t24615);
        let (t94391, t94395) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2090(t24637, t8009, t24588, t8020);
    (t94354, t94358, t94363, t94365, t94369, t94374, t94378, t94391, t94395)
}
