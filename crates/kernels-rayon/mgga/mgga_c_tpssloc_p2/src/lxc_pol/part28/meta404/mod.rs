//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta404 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1564;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1565;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1566;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta404(t1864: f64, t645: f64, t192: f64, t532: f64, t1982: f64, t3701: f64, t3914: f64, t1390: f64, t3719: f64, t3734: f64, t191: f64, t3660: f64, t1887: f64, t6916: f64, t213: f64, t225: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22550, t22573, t22574) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1564(t1864, t645, t192, t532, t1982);
        let (t22578, t22584, t22596, t22607, t22633) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1565(t3701, t3914, t1390, t3719, t3734, t191, t192, t3660, t1887, t6916);
        let t22635 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1566(t213, t225, t562);
    (t22550, t22573, t22574, t22578, t22584, t22596, t22607, t22633, t22635)
}
