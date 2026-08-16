//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta236 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk961;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk962;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk963;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk964;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk965;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta236(t1088: f64, t11163: f64, t123: f64, t3247: f64, t607: f64, t2250: f64, t1089: f64, t9258: f64, t11136: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11150: f64, t11156: f64, t11161: f64, t449: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11164, t11165) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk961(t1088, t11163, t123);
        let (t11167, t11168) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk962(t3247, t607, t2250);
        let (t11169, t11170) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk963(t1088, t11168, t123);
        let t11172 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk964(t1089, t9258);
        let (t11173, t11174) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk965(t1088, t11172, t123);
        let (t11176, t11177) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk966(t11136, t11137, t11139, t11141, t11143, t11150, t11156, t11161, t11165, t11170, t11174, t449);
    (t11164, t11165, t11167, t11168, t11169, t11170, t11172, t11173, t11174, t11176, t11177)
}
