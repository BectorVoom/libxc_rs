//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta621(t1863: f64, t83728: f64, t1864: f64, t2307: f64, t22522: f64, t9239: f64, t9231: f64, t2240: f64, t22511: f64, t33: f64, t39049: f64, t6489: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t83729, t83738, t83741, t83750, t83760, t83775) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2100(t1863, t83728, t1864, t2307, t22522, t9239, t9231, t2240, t22511, t33, t39049, t6489);
    (t83729, t83738, t83741, t83750, t83760, t83775)
}
