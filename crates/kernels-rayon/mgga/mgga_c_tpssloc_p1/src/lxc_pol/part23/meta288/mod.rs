//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta288 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk997;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk998;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta288(t10216: f64, t20234: f64, t10304: f64, t136: f64, t20217: f64, t883: f64, t908: f64, t2770: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let t21130 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk997(t10216, t20234);
        let (t21131, t21132, t21134) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk998(t10304, t21130, t136, t20217, t883);
        let (t21135, t21136, t21138) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk999(t21134, t908, t136, t20234, t2770);
    (t21130, t21131, t21132, t21134, t21135, t21136, t21138)
}
