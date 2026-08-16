//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta274 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1035;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1036;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1037;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta274(t1285: f64, t2221: f64, t1287: f64, t522: f64, t9216: f64, t9218: f64, t1294: f64, t9713: f64, t25: f64, t526: f64, t3664: f64, t606: f64, t28: f64, t11988: f64, t2249: f64, t514: f64, t9257: f64, t528: f64, t1081: f64, t3672: f64, t11122: f64, t12001: f64, t3231: f64, t517: f64, zeta_threshold: f64, t157: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12051, t12053, t12055, t12057, t12059, t12061, t12064) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1035(t1285, t2221, t1287, t522, t9216, t9218, t1294, t9713, t25, t526, t3664, t606);
        let (t12070, t12072, t12075, t12081) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1036(t25, t28, t11988, t12061, t12064, t2249, t514, t9257, t528, t1081, t3672, t11122, t12001, t3231, t517, zeta_threshold);
        let t12083 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1037(t12070, t12081, t157);
    (t12051, t12053, t12055, t12057, t12059, t12061, t12064, t12072, t12075, t12083)
}
