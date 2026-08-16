//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1352;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta462(t136: f64, t2826: f64, t76597: f64, t76593: f64, t41880: f64, t76572: f64, t76576: f64, t908: f64, t76589: f64, t10304: f64, t76581: f64, t76585: f64, t68500: f64, t68502: f64, t68504: f64, t68506: f64, t76624: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76877, t76880, t76887, t76890, t76893, t76896, t76899) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1352(t136, t2826, t76597, t76593, t41880, t76572, t76576, t908, t76589, t10304, t76581, t76585);
        let (t76901, t76903) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1353(t68500, t68502, t68504, t68506, t76877, t76880, t76887, t76890, t76893, t76896, t76899, t136, t76624, t908);
    (t76877, t76880, t76887, t76890, t76893, t76896, t76899, t76901, t76903)
}
