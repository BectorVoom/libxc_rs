//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2182;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta602(t11721: f64, t23508: f64, t1009: f64, t11598: f64, t1243: f64, t11714: f64, t476: f64, t42341: f64, t44696: f64, t3508: f64, t3502: f64, t1209: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44701, t44706, t44707, t44722, t44724, t44725, t44726, t44753, t44754, t44785) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2182(t11721, t23508, t1009, t11598, t1243, t11714, t476, t42341, t44696, t3508, t3502, t1209);
    (t44701, t44706, t44707, t44722, t44724, t44725, t44726, t44753, t44754, t44785)
}
