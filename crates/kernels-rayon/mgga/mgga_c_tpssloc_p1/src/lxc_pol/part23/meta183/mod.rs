//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta183 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk812;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta183(t10216: f64, t344: f64, t698: f64, t976: f64, t135: f64, t2978: f64, t2770: f64, t343: f64, t2775: f64, t2769: f64, t40: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t10217, t10224, t10231, t10236, t10254, t10276, t10277) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk812(t10216, t344, t698, t976, t135, t2978, t2770, t343, t2775, t2769, t40);
    (t10217, t10224, t10231, t10236, t10254, t10276, t10277)
}
